use crate::header::Header;
use crate::query::Question;
use crate::{Deserializable, Deserializer, QueryType, Serializable, Serializer};

/// DNS 协议通信消息
///
///     +---------------------+
///     |        Header       |
///     +---------------------+
///     |       Question      | the question for the name server
///     +---------------------+
///     |        Answer       | RRs answering the question
///     +---------------------+
///     |      Authority      | RRs pointing toward an authority
///     +---------------------+
///     |      Additional     | RRs holding additional information
///     +---------------------+
///
/// 参考：[RFC1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1)
#[derive(Debug)]
pub struct Message {
    header: Header,
    question: Question,
    // answer: Answer,
}

impl Message {
    pub fn new(query_id: u16, domain: &str, query_type: QueryType) -> Self {
        Self {
            header: Header::new(query_id, false, 0, false, false, true, false, 0, 1, 0, 0, 0),
            question: Question::new(domain, query_type),
        }
    }
}

impl Serializable for Message {
    fn serialize(&self, serializer: &mut Serializer) {
        self.header.serialize(serializer);
        self.question.serialize(serializer);
    }
}

impl Deserializable<'_> for Message {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            header: Header::deserializable(deserializer).unwrap(),
            question: Question::deserializable(deserializer).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;
    use crate::message::Message;
    use crate::query::Question;
    use crate::{QueryType, deserialize, serialize};

    #[test]
    fn test_serialize() {
        let message = Message::new(0xb962, "example.com", QueryType::A);
        let encoded = serialize(&message);

        assert_eq!(
            encoded,
            [
                0xb9u8, 0x62u8, 0x01u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8, 0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8,
                0x03u8, 0x63u8, 0x6fu8, 0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8
            ]
        )
    }

    #[test]
    fn test_deserialize() {
        let encoded = [
            0xb9u8, 0x62u8, 0x01u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8, 0x00u8, 0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8,
            0x03u8, 0x63u8, 0x6fu8, 0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8
        ];
        let message: Message = deserialize(&encoded);

        assert_eq!(message.question, Question::new("example.com", QueryType::A));
        assert_eq!(message.header, Header::new(0xb962, false, 0, false, false, true, false, 0, 1, 0, 0, 0))
    }
}
