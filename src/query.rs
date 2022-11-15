use rand::Rng;
use packed_struct::prelude::*;

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
#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
struct QueryHeader {
    #[packed_field(bits="0..=1")]
    query_id: u16,
    #[packed_field(bits="2..=3")]
    flag: u16,
    #[packed_field(bits="4..=5")]
    num_questions: u16,
    #[packed_field(bits="6..=7")]
    num_answers: u16,
    #[packed_field(bits="7..=8")]
    num_auth: u16,
    #[packed_field(bits="9..=10")]
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

struct QueryQuestion {
    query_class: u16,
    query_type: u16,
    domain: Vec<u8>,
}

impl QueryQuestion {
    fn new(domain: &str, query_type: u16) -> Self {
        Self {
            query_class: 1,
            query_type,
            domain: Vec::new(),
        }
    }
}
