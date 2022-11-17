use crate::query::QueryType::A;
use bincode::enc::write::Writer;
use bincode::enc::Encoder;
use bincode::error::EncodeError;
use bincode::Encode;
use crate::header::QueryHeader;


/// 查询类型
#[derive(Encode)]
pub enum QueryType {
    /// A 记录
    A,
}

/// DNS 查询结构
#[derive(Encode)]
pub struct Query {
    header: QueryHeader,
    question: QueryQuestion,
}

impl Query {
    pub fn new(query_id: u16, domain: &str, query_type: QueryType) -> Self {
        Self {
            header: QueryHeader::new(query_id, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000),
            question: QueryQuestion::new(domain, query_type),
        }
    }
}

struct QueryQuestion {
    domain: Vec<u8>,
    query_type: QueryType,
    query_class: u16,
}

impl QueryQuestion {
    fn new(domain: &str, query_type: QueryType) -> Self {
        Self {
            query_class: 1,
            query_type,
            domain: QueryQuestion::encode_domain(domain),
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
            result.extend(part.as_bytes())
        }
        result.extend("\0".as_bytes());
        result
    }
}

impl bincode::Encode for QueryQuestion {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&self.domain)?;
        match self.query_type {
            A => 1u16.encode(encoder)?,
        };
        self.query_class.encode(encoder)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::query::{Query, QueryQuestion, QueryType};
    use crate::serialize_to_bytes;

    #[test]
    fn test_encode_domain() {
        let encoded = QueryQuestion::encode_domain("example.com");
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
        let q_question = QueryQuestion::new("example.com", QueryType::A);
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
    fn test_query_to_bytes() {
        let query = Query::new(0xb962, "example.com", QueryType::A);
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
