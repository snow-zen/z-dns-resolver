mod answer;
mod header;
mod query;

use bincode::config::{BigEndian, Configuration, Fixint};
use bincode::{config, de, enc};
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

use crate::query::{Query, QueryType};
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

    let query = Query::new(
        rand::thread_rng().gen_range(0..65535),
        &args.domain,
        QueryType::A,
    );
    socket
        .send(&serialize_to_bytes(&query))
        .expect("send message fail");

    let mut buf = [0; 1024 * 4]; // 4k
    let (number_of_buf, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

    println!("{:?}", Vec::from(&buf[..number_of_buf]));
    Ok(())
}

const BIN_CODE_CONFIG: Configuration<BigEndian, Fixint> = config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub fn serialize_to_bytes<E>(t: &E) -> Vec<u8>
where
    E: enc::Encode,
{
    bincode::encode_to_vec(t, BIN_CODE_CONFIG).unwrap()
}

pub fn deserialize_to_struct<D>(bytes: &[u8]) -> (D, usize)
where
    D: de::Decode,
{
    bincode::decode_from_slice(bytes, BIN_CODE_CONFIG).unwrap()
}
