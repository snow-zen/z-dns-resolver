use crate::{
    decode_domain, encode_domain, Deserializable, Deserializer, Serializable, Serializer,
};

/// 资源记录类型
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum ResourceRecordType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

impl From<u16> for ResourceRecordType {
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x) }
    }
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    pub fn get_rdata(&self) -> &str {
        &self.rdata
    }

    pub fn get_type_name(&self) -> &str {
        match self.rr_type {
            ResourceRecordType::A => "A",
            ResourceRecordType::NS => "NS",
            ResourceRecordType::MD => "MD",
            ResourceRecordType::MF => "MF",
            ResourceRecordType::CNAME => "CNAME",
            ResourceRecordType::SOA => "SOA",
            ResourceRecordType::MB => "MB",
            ResourceRecordType::MG => "MG",
            ResourceRecordType::MR => "MR",
            ResourceRecordType::NULL => "NULL",
            ResourceRecordType::WKS => "WKS",
            ResourceRecordType::PTR => "PTR",
            ResourceRecordType::HINFO => "HINFO",
            ResourceRecordType::MINFO => "MINFO",
            ResourceRecordType::MX => "MX",
            ResourceRecordType::TXT => "TXT"
        }
    }

    fn decode_rdata(
        deserializer: &mut Deserializer,
        rr_type: &ResourceRecordType,
        rdlength: u16,
    ) -> String {
        match rr_type {
            ResourceRecordType::A => deserializer
                .read_slice3(rdlength as usize)
                .into_iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join("."),
            ResourceRecordType::NS | ResourceRecordType::CNAME => decode_domain(deserializer, 0),
            _ => String::from_utf8(deserializer.read_slice3(rdlength as usize)).unwrap(),
        }
    }
}

impl Serializable for ResourceRecord {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.extend(encode_domain(&self.name));
        serializer.extend((self.rr_type as u16).to_be_bytes());
        serializer.extend(self.class.to_be_bytes());
        serializer.extend(self.ttl.to_be_bytes());
        serializer.extend(self.rdlength.to_be_bytes());
        serializer.extend(self.rdata.as_bytes().to_owned());
    }
}

impl Deserializable for ResourceRecord {
    fn deserializable(deserializer: &mut Deserializer) -> Option<Self>
    where
        Self: Sized,
    {
        let name = decode_domain(deserializer, 0);
        let rr_type = ResourceRecordType::from(u16::deserializable(deserializer)?);
        let class = u16::deserializable(deserializer)?;
        let ttl = u32::deserializable(deserializer)?;
        let rdlength = u16::deserializable(deserializer)?;
        let rdata = ResourceRecord::decode_rdata(deserializer, &rr_type, rdlength);
        Some(Self {
            name,
            rr_type,
            class,
            ttl,
            rdlength,
            rdata,
        })
    }
}
