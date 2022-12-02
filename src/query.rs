use crate::query::QueryType::A;
use crate::{decode_domain, Deserializable, Deserializer, encode_domain, Serializable, Serializer};

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

/// DNS 消息结构 Query 部分
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
}

impl Serializable for Question {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.extend(encode_domain(&self.qname));
        match self.qtype {
            A => serializer.extend((A as u16).to_be_bytes()),
        };
        serializer.extend(self.qclass.to_be_bytes());
    }
}

impl Deserializable<'_> for Question {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self>
    where
        Self: Sized,
    {
        let qname = decode_domain(deserializer, 0);
        let qtype = QueryType::from(u16::from_be_bytes(deserializer.read_slice::<2>()));
        let qclass = u16::from_be_bytes(deserializer.read_slice::<2>());
        Some(Self {
            qname,
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
