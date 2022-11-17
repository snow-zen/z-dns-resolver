use crate::header::Header;
use crate::query::Question;
use crate::QueryType;
use bincode::{Decode, Encode};

/// DNS 协议通信消息
#[derive(Encode, Decode)]
pub struct Message {
    header: Header,
    question: Question,
    // answer: Answer,
}

impl Message {
    pub fn new(query_id: u16, domain: &str, query_type: QueryType) -> Self {
        Self {
            header: Header::new(query_id, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000),
            question: Question::new(domain, query_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{QueryType, serialize_to_bytes};
    use crate::message::Message;

    #[test]
    fn test_message_to_bytes() {
        let query = Message::new(0xb962, "example.com", QueryType::A);
        let encoded = serialize_to_bytes(&query);

        assert_eq!(
            encoded,
            [
                0xb9u8, 0x62u8, 0x01u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8, 0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8,
                0x03u8, 0x63u8, 0x6fu8, 0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8
            ]
        )
    }
}
