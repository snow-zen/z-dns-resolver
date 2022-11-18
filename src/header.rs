use bincode::de::{BorrowDecoder, Decoder};
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};

/// DNS 查询结构 Header 部分，结构数据的最大长度为 12 字节。以下是以位为单位的数据结构示意：
///
///                                     1  1  1  1  1  1
///       0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                      ID                       |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    QDCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    ANCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    NSCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    ARCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
/// 参考：[RFC1035](https://www.rfc-editor.org/rfc/pdfrfc/rfc1035.txt.pdf)
pub struct Header {
    /// 标识符。
    ///
    /// 占用 16 位。
    id: u16,
    /// 用于指定当前消息是 query(0) 还是 response(1)。
    ///
    /// 占用 1 位。
    qr: bool,
    /// 指定消息中的查询类型，其值有以下定义
    /// 0       = 标准查询（QUERY）
    /// 1       = 反向查询（IQUERY）
    /// 2       = 服务器状态请求（STATUS）
    /// 3-15    = 保留值
    ///
    /// 占用 4 位。
    opcode: u8,
    /// 权威答案，该位在响应中有效，表示响应名称服务器是部分域名的权威服务器。
    ///
    /// 注意：[Answer] 部分的内容由于别名可能有多个所有者名称。
    /// aa 对应与查询名称匹配的名称，或者 [Answer] 部分第一个所有者名称。
    ///
    /// 占用 1 位。
    aa: bool,
    /// 指定该消息由于长度大于传输信道允许长度而被截断。
    ///
    /// 占用 1 位。
    tc: bool,
    /// 可在查询中设置并复制到响应中。如果已设置，它表示名称服务器递归执行查询。
    ///
    /// 占用 1 位。
    rd: bool,
    /// 可在响应中设置或者清除，表示名称服务器是否提供递归查询。
    ///
    /// 占用 1 位。
    ra: bool,
    /// 保留字段供将来使用，在查询和响应中必须全为 0。
    ///
    /// 占用 3 位。
    z: u8,
    /// 响应代码，值为：
    /// 0       = 无错误。
    /// 1       = 格式错误，名称服务器无法解析查询。
    /// 2       = 服务器失败，由于名称服务器出现问题，造成查询无法处理。
    /// 3       = 名称错误，仅对于来自权威名称服务器的响应有意义，表示查询中引用的域名不存在。
    /// 4       = 没有实现，名称服务器不支持查询的请求类型。
    /// 5       = 拒绝，由于策略原因，名称服务器拒绝执行指定操作。
    /// 6-15    = 保留供将来使用。
    ///
    /// 占用 4 位
    rcode: u8,
    /// 指定 [Question] 中的条目数量。
    ///
    /// 占用 16 位。
    qdcount: u16,
    /// 指定 [Answer] 中的资源记录条目数量。
    ///
    /// 占用 16 位。
    ancount: u16,
    /// 指定 [AuthorityRecords] 中的服务器资源记录条目数量。
    ///
    /// 占用 16 位。
    nscount: u16,
    /// 指定 [AdditionalRecords] 中的资源记录条目数量。
    ///
    /// 占用 16 位。
    arcount: u16,
}

impl Header {
    pub fn new(
        id: u16,
        qr: bool,
        opcode: u8,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        rcode: u8,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Self {
        Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
            z: 0b000,
        }
    }

    // fn do_encode()
}

impl Encode for Header {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.id.encode(encoder)?;
        let mut flag: u16 = self.qr.into();
        flag = (flag << 4) | u16::from(self.opcode & 0xfu8);
        flag = (flag << 1) | u16::from(self.aa);
        flag = (flag << 1) | u16::from(self.tc);
        flag = (flag << 1) | u16::from(self.rd);
        flag = (flag << 1) | u16::from(self.ra);
        flag = (flag << 3) | u16::from(self.z);
        flag = (flag << 4) | u16::from(self.rcode);
        flag.encode(encoder)?;
        self.qdcount.encode(encoder)?;
        self.ancount.encode(encoder)?;
        self.nscount.encode(encoder)?;
        self.arcount.encode(encoder)?;
        Ok(())
    }
}

impl Decode for Header {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(12)?;
        let id = u16::decode(decoder)?;
        let mut flag = u16::decode(decoder)?;

        let rcode = u8::try_from(flag & 0xfu16).unwrap();
        flag >>= 4;
        let z = u8::try_from(flag & 0x7u16).unwrap();
        flag >>= 3;
        let ra = (flag & 0x1u16) != 0;
        flag >>= 1;
        let rd = (flag & 0x1u16) != 0;
        flag >>= 1;
        let tc = (flag & 0x1u16) != 0;
        flag >>= 1;
        let aa = (flag & 0x1u16) != 0;
        flag >>= 1;
        let opcode = u8::try_from(flag & 0xfu16).unwrap();
        flag >>= 4;
        let qr = (flag & 0x1u16) != 0;
        let qdcount = u16::decode(decoder)?;
        let ancount = u16::decode(decoder)?;
        let nscount = u16::decode(decoder)?;
        let arcount = u16::decode(decoder)?;

        Ok(Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        })
    }
}

impl<'de> BorrowDecode<'de> for Header {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Header::decode(decoder)
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;
    use crate::{deserialize_to_struct, serialize_to_bytes};

    #[test]
    fn test_query_header_to_bytes() {
        let q_header = Header::new(0xb962, false, 0, false, false, true, false, 0, 1, 0, 0, 0);
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
        assert_eq!(q_header.qr, false);
        assert_eq!(q_header.opcode, 0);
        assert_eq!(q_header.aa, false);
        assert_eq!(q_header.tc, false);
        assert_eq!(q_header.rd, true);
        assert_eq!(q_header.ra, false);
        assert_eq!(q_header.rcode, 0);
        assert_eq!(q_header.qdcount, 1);
        assert_eq!(q_header.ancount, 0);
        assert_eq!(q_header.nscount, 0);
        assert_eq!(q_header.arcount, 0);
    }
}
