use bincode::Options;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn serialize_to_bytes<S>(t: &S) -> Vec<u8>
where
    S: ?Sized + serde::Serialize,
{
    bincode::options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(t)
        .unwrap()
}

/// 查询类型
pub enum QueryType {
    /// A 记录
    A = 1,
}

/// DNS 查询结构
pub struct Query {
    header: QueryHeader,
    question: QueryQuestion,
}

impl Query {
    pub fn new(domain: &str, query_type: u16) -> Self {
        Self {
            header: QueryHeader::new(
                rand::thread_rng().gen_range(0..65535),
                0x0100,
                0x0001,
                0x0000,
                0x0000,
                0x0000,
            ),
            question: QueryQuestion::new(domain, query_type),
        }
    }
}

/// DNS 查询结构 Header 部分
#[derive(Serialize)]
struct QueryHeader {
    query_id: u16,
    flag: u16,
    num_questions: u16,
    num_answers: u16,
    num_auth: u16,
    num_additional: u16,
}

impl QueryHeader {
    fn new(
        query_id: u16,
        flag: u16,
        num_questions: u16,
        num_answers: u16,
        num_auth: u16,
        num_additional: u16,
    ) -> Self {
        Self {
            query_id,
            flag,
            num_questions,
            num_answers,
            num_auth,
            num_additional,
        }
    }
}

#[derive(Serialize)]
struct QueryQuestion {
    domain: Vec<u8>,
    query_type: u16,
    query_class: u16,
}

impl QueryQuestion {
    fn new(domain: &str, query_type: u16) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::query::{QueryHeader, QueryQuestion, serialize_to_bytes};
    use bincode::Options;

    #[test]
    fn test_query_header_to_bytes() {
        let q_header = QueryHeader::new(0xb962, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000);
        let encoded = serialize_to_bytes(&q_header);

        assert_eq!(encoded.len(), 12);
        assert_eq!(
            encoded,
            [
                0xb9u8, 0x62u8, 0x01u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
                0x00u8, 0x00u8
            ]
        );
    }

    fn test_query_question_to_bytes() {
        let q_question = QueryQuestion::new("example.com", 1);
        let encoded = serialize_to_bytes(&q_question);

        assert_eq!(
            encoded,
            [
                0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8,
                0x6fu8, 0x6du8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x01u8
            ]
        )
    }
}
