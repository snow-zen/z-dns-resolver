use crate::query::QueryType::A;
use crate::{Deserializable, Deserializer, Serializable, Serializer};

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

/// DNS 查询结构 Query 部分
///
///                                     1  1  1  1  1  1
///       0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                                               |
///     /                     QNAME                     /
///     /                                               /
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                     QTYPE                     |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                     QCLASS                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
/// 参考：[RFC1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2)
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

impl Serializable for Question {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.extend(Question::encode_domain(&self.qname));
        match self.qtype {
            A => serializer.extend((A as u16).to_be_bytes()),
        };
        serializer.extend(self.qclass.to_be_bytes());
    }
}

impl Deserializable<'_> for Question {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self> where Self: Sized {
        let mut qname = Vec::new();
        loop {
            let label_len = deserializer.read();
            if label_len == b'\0' {
                // end
                break;
            }
            if label_len == 0b11000000 {
                // DNS compression! need decompression
                // let offset = usize::from_be_bytes([label_len & 0x3f, u8::decode(decoder)?]);
                // decompression_domain_from_slice(decoder.reader().peek_read(), )
            }
            let mut label_buf = Vec::new();
            for _ in 0..label_len {
                label_buf.push(deserializer.read());
            }
            qname.push(String::from_utf8(label_buf).unwrap())
        }

        let qtype = QueryType::from(u16::from_be_bytes(deserializer.read_slice::<2>()));
        let qclass = u16::from_be_bytes(deserializer.read_slice::<2>());
        Some(Self {
            qname: qname.join("."),
            qtype,
            qclass,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::query::{QueryType, Question};
    use crate::{deserialize, serialize};

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
    fn test_serialize() {
        let q_question = Question::new("example.com", QueryType::A);
        let encoded = serialize(&q_question);

        assert_eq!(
            encoded,
            [
                0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8,
                0x6fu8, 0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8
            ]
        )
    }

    #[test]
    fn test_deserialize() {
        let bytes = [
            0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8, 0x6fu8,
            0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8,
        ];
        let question: Question = deserialize(&bytes);

        assert_eq!(question.qname, "example.com");
        assert_eq!(question.qtype, QueryType::A);
        assert_eq!(question.qclass, 1);
    }
}
