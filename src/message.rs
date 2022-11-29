use crate::header::Header;
use crate::query::Question;
use crate::QueryType;
use bincode::de::Decoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bincode::enc::Encoder;

/// DNS 协议通信消息
// #[derive(Encode, Decode)]
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

impl Encode for Message {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.header.encode(encoder)?;
        self.question.encode(encoder)?;
        Ok(())
    }
}

impl Decode for Message {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self {
            header: Header::decode(decoder)?,
            question: Question::decode(decoder)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::Message;
    use crate::{serialize_to_bytes, QueryType};

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
