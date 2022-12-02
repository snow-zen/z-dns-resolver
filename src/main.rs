mod answer;
mod de;
mod header;
mod message;
mod query;
mod se;

use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

use crate::de::{Deserializable, Deserializer};
use crate::message::Message;
use crate::query::QueryType;
use crate::se::{Serializable, Serializer};
use clap::Parser;
use rand::Rng;

const MAX_COMPRESSION_COUNT: u8 = 126;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Specify domain
    domain: String,
    /// Specify domain server host
    #[arg(short = 'D', long, default_value = "114.114.114.114")]
    dns_server: String,
    /// Specify domain server port
    #[arg(short, long, default_value = "53", value_parser = clap::value_parser!(u16).range(0..=65535))]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    // 端口号为 0 表示由底层系统分配端口
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

    socket
        .connect((Ipv4Addr::from_str(&args.dns_server).unwrap(), args.port))
        .expect("connect fail");

    let request_message = Message::new(
        rand::thread_rng().gen_range(0..65535),
        &args.domain,
        QueryType::A,
    );
    socket
        .send(&serialize(&request_message))
        .expect("send message fail");

    let mut buf = [0; 1024 * 4]; // 4k
    let (number_of_buf, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

    let resp_message: Message = deserialize(&buf[..number_of_buf]);
    println!("{:?}", resp_message);
    Ok(())
}

pub fn serialize<S>(src: &S) -> Vec<u8>
where
    S: Serializable,
{
    let mut serializer = Serializer::new();
    src.serialize(&mut serializer);
    serializer.to_owned_bytes()
}

pub fn deserialize<'d, D>(src: &[u8]) -> D
where
    D: Deserializable<'d>,
{
    let mut deserializer = Deserializer::new(src);
    D::deserializable(&mut deserializer).unwrap()
}

pub fn encode_domain(domain: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for part in domain.split('.') {
        result.extend(
            u8::try_from(part.len())
                .expect("domain part is too long")
                .to_be_bytes(),
        );
        result.extend(part.as_bytes());
    }
    result.push(b'\0');
    result
}

pub fn decode_domain(deserializer: &mut Deserializer, recursion_count: u8) -> String {
    let mut result = Vec::new();
    loop {
        let label_len = deserializer.read();
        if label_len == b'\0' {
            // end
            break;
        }
        if label_len == 0b11000000 {
            // DNS compression! need decompression
            if recursion_count > MAX_COMPRESSION_COUNT {
                panic!("Too many compression pointer!")
            }
            let offset = u16::from_be_bytes([label_len & 0x3f, deserializer.read()]);
            let old_cursor = deserializer.reset_cursor(offset as usize);
            result.push(decode_domain(deserializer, recursion_count + 1));
            deserializer.reset_cursor(old_cursor);
            break;
        }
        let mut label_buf = Vec::new();
        for _ in 0..label_len {
            label_buf.push(deserializer.read());
        }
        result.push(String::from_utf8(label_buf).unwrap())
    }
    result.join(".")
}

#[cfg(test)]
mod tests {
    use crate::encode_domain;

    #[test]
    fn test_encode_domain() {
        let encoded = encode_domain("example.com");
        assert_eq!(
            encoded,
            [
                0x07u8, 0x65u8, 0x78u8, 0x61u8, 0x6du8, 0x70u8, 0x6cu8, 0x65u8, 0x03u8, 0x63u8,
                0x6fu8, 0x6du8, 0x00u8
            ]
        )
    }
}