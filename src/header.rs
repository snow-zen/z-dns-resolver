use bincode::{Decode, Encode};

/// DNS 查询结构 Header 部分
#[derive(Encode, Decode)]
pub struct QueryHeader {
    query_id: u16,
    flag: u16,
    num_questions: u16,
    num_answers: u16,
    num_auth: u16,
    num_additional: u16,
}

impl QueryHeader {
    pub fn new(
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

#[cfg(test)]
mod tests {
    use crate::header::QueryHeader;
    use crate::{deserialize_to_struct, serialize_to_bytes};

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

    #[test]
    fn test_bytes_to_query_header() {
        let bytes = [
            0xb9u8, 0x62u8, 0x01u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
            0x00u8,
        ];
        let (q_header, number_size) = deserialize_to_struct::<QueryHeader>(&bytes);

        assert_eq!(number_size, 12);
        assert_eq!(q_header.query_id, 0xb962);
        assert_eq!(q_header.flag, 0x0100);
        assert_eq!(q_header.num_questions, 0x0001);
        assert_eq!(q_header.num_answers, 0x0000);
        assert_eq!(q_header.num_auth, 0x0000);
        assert_eq!(q_header.num_additional, 0x0000);
    }
}
