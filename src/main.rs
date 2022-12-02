mod answer;
mod de2;
mod header;
mod message;
mod query;
mod se;

use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

use crate::de2::{Deserializable, Deserializer};
use crate::message::Message;
use crate::query::QueryType;
use crate::se::{Serializable, Serializer};
use clap::Parser;
use rand::Rng;

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
        .send(&Vec::new())
        // .send(&serialize_to_bytes(&request_message))
        .expect("send message fail");

    let mut buf = [0; 1024 * 4]; // 4k
    let (number_of_buf, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

    println!("{:?}", Vec::from(&buf[..number_of_buf]));
    Ok(())
}

pub fn decompression_domain_from_slice(bytes: &[u8], mut offset: usize) -> String {
    let mut result = Vec::new();
    loop {
        let label_len = bytes[offset];
        offset += 1;
        if label_len == b'\0' {
            // end
            break;
        }

        let mut label_buf = Vec::new();
        for _ in 0..label_len {
            label_buf.push(bytes[offset]);
            offset += 1;
        }
        result.push(String::from_utf8(label_buf).unwrap());
    }
    result.join(".")
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
