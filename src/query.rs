use crate::query::QueryType::A;
use bincode::de::Decoder;
use bincode::de::read::Reader;
use bincode::enc::write::Writer;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use crate::decompression_domain_from_slice;

/// 查询类型
#[repr(u16)]
#[derive(PartialEq, Debug)]
pub enum QueryType {
    /// A 记录
    A = 1,
}

impl From<u16> for QueryType {
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x) }
    }
}

#[derive(PartialEq, Debug)]
pub struct Question {
    qname: String,
    qtype: QueryType,
    qclass: u16,
}

impl Question {
    pub fn new(domain: &str, query_type: QueryType) -> Self {
        Self {
            qclass: 1,
            qtype: query_type,
            qname: String::from(domain),
        }
    }

    fn encode_domain(domain: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for part in domain.split('.') {
            result.extend(
                u8::try_from(part.len())
                    .expect("domain part is too long")
                    .to_be_bytes(),
            );
            result.extend(part.as_bytes());
        }
        result.push(b'\0');
        result
    }
}

impl bincode::Encode for Question {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder
            .writer()
            .write(&Question::encode_domain(&self.qname))?;
        match self.qtype {
            A => (A as u16).encode(encoder)?,
        };
        self.qclass.encode(encoder)?;
        Ok(())
    }
}

impl bincode::Decode for Question {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut qname = Vec::new();
        loop {
            let label_len = u8::decode(decoder)?;
            if label_len == b'\0' {
                // end
                break;
            }
            if label_len == 0b11000000 {
                // DNS compression! need decompression
                let offset = usize::from_be_bytes([label_len & 0x3f, u8::decode(decoder)?]);
                decompression_domain_from_slice(decoder.reader().peek_read(), )
            }
            let mut label_buf = Vec::new();
            for _ in 0..label_len {
                label_buf.push(u8::decode(decoder)?);
            }
            qname.push(String::from_utf8(label_buf).unwrap())
        }

        let qtype = QueryType::from(u16::decode(decoder)?);
        let qclass = u16::decode(decoder)?;
        Ok(Self {
            qname: qname.join("."),
            qtype,
            qclass,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::query::{QueryType, Question};
    use crate::{deserialize_to_struct, serialize_to_bytes};

    #[test]
    fn test_encode_domain() {
        let encoded = Question::encode_domain("example.com");
        assert_eq!(
            encoded,
            [
                0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8,
                0x6fu8, 0x6du8, 0x00u8
            ]
        )
    }

    #[test]
    fn test_query_question_to_bytes() {
        let q_question = Question::new("example.com", QueryType::A);
        let encoded = serialize_to_bytes(&q_question);

        assert_eq!(
            encoded,
            [
                0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8,
                0x6fu8, 0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8
            ]
        )
    }

    #[test]
    fn test_bytes_to_question() {
        let bytes = [
            0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8, 0x6fu8,
            0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8,
        ];
        let (question, number_size) = deserialize_to_struct::<Question>(&bytes);

        assert_eq!(number_size, bytes.len());
        assert_eq!(question.qname, "example.com");
        assert_eq!(question.qtype, QueryType::A);
        assert_eq!(question.qclass, 1);
    }
}
