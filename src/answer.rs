use crate::{decode_domain, Deserializable, Deserializer, QueryType};

/// 资源记录类型
#[repr(u16)]
pub enum ResourceRecordType {
    A = 1,
    NS = 2,
    // MD = 3,
    // MF = 4,
    CNAME = 5,
    // SOA = 6,
    // MB = 7,
    // MG = 8,
    // MR = 9,
    // NULL = 10,
    // WKS = 11,
    // PTR = 12,
    // HINFO = 13,
    // MINFO = 14,
    // MX = 15,
    // TXT = 16,
}

/// DNS 消息结构中 Answer、Authority、Additional 部分都使用该形式
///
///                                     1  1  1  1  1  1
///       0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                                               |
///     /                                               /
///     /                      NAME                     /
///     |                                               |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                      TYPE                     |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                     CLASS                     |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                      TTL                      |
///     |                                               |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                   RDLENGTH                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
///     /                     RDATA                     /
///     /                                               /
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
/// 参考：[RFC1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.3)
#[derive(Debug)]
pub struct ResourceRecord {
    name: String,
    rr_type: ResourceRecordType,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: String,
}

impl From<u16> for ResourceRecord {
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x) }
    }
}

impl ResourceRecord {
    pub fn new(
        name: String,
        rr_type: ResourceRecordType,
        class: u16,
        ttl: u32,
        rdlength: u16,
        rdata: String,
    ) -> Self {
        Self {
            name,
            rr_type,
            class,
            ttl,
            rdlength,
            rdata,
        }
    }

    fn decode_rdata(
        deserializer: &mut Deserializer,
        rr_type: ResourceRecordType,
        rdlength: u16,
    ) -> String {
        match rr_type {
            ResourceRecordType::A => {
                let rdate = deserializer.read_slice();
                String::new()
            },
            ResourceRecordType::NS | ResourceRecordType::CNAME => {
                decode_domain(deserializer, 0)
            }
        }
    }
}

impl Deserializable<'_> for ResourceRecord {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self>
    where
        Self: Sized,
    {
        let name = decode_domain(deserializer, 0);
        let rr_type = u16::from_be_bytes(deserializer.read_slice::<2>());
        let class = u16::from_be_bytes(deserializer.read_slice::<2>());
        let ttl = u32::from_be_bytes(deserializer.read_slice::<4>());
        let rdlength = u16::from_be_bytes(deserializer.read_slice::<2>());
    }
}
