use bincode::{Decode, Encode};

/// DNS 查询结构 Header 部分
#[derive(Encode, Decode)]
pub struct Header {
    /// 标识符
    id: u16,
    /// 用于指定当前消息是 query(0) 还是 response(1)
    qr: bool,
    /// 指定消息中的查询类型，其值有以下定义
    /// 0       = 标准查询（QUERY）
    /// 1       = 反向查询（IQUERY）
    /// 2       = 服务器状态请求（STATUS）
    /// 3-15    = 保留值
    opcode: u8,
    /// 权威答案，该位在响应中有效，表示响应名称服务器
    ///
    /// 注意：[Answer] 部分的内容由于别名可能有多个所有者名称。
    /// aa 对应与查询名称匹配的名称，或者 [Answer] 部分第一个所有者名称。
    aa: u8,

    // flag: u16,
    // num_questions: u16,
    // num_answers: u16,
    // num_auth: u16,
    // num_additional: u16,
}

impl Header {
    pub fn new(
        query_id: u16,
        flag: u16,
        num_questions: u16,
        num_answers: u16,
        num_auth: u16,
        num_additional: u16,
    ) -> Self {
        Self {
            id: query_id,
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
    use crate::header::Header;
    use crate::{deserialize_to_struct, serialize_to_bytes};

    #[test]
    fn test_query_header_to_bytes() {
        let q_header = Header::new(0xb962, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000);
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
        let (q_header, number_size) = deserialize_to_struct::<Header>(&bytes);

        assert_eq!(number_size, 12);
        assert_eq!(q_header.id, 0xb962);
        assert_eq!(q_header.flag, 0x0100);
        assert_eq!(q_header.num_questions, 0x0001);
        assert_eq!(q_header.num_answers, 0x0000);
        assert_eq!(q_header.num_auth, 0x0000);
        assert_eq!(q_header.num_additional, 0x0000);
    }
}
