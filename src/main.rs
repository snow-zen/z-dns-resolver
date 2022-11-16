mod query;

use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

use crate::query::{serialize_to_bytes, Query, QueryType};
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    /// Specify domain
    #[arg(short, long)]
    domain: String,
}

const DNS_PROTOCOL_DEFAULT_PORT: u16 = 53;
const DEFAULT_DNS_SERVER_HOST: &str = "114.114.114.114";

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    // 端口号为 0 表示由底层系统分配端口
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

    socket
        .connect((
            Ipv4Addr::from_str(DEFAULT_DNS_SERVER_HOST).unwrap(),
            DNS_PROTOCOL_DEFAULT_PORT,
        ))
        .expect("connect fail");

    let query = Query::new(
        rand::thread_rng().gen_range(0..65535),
        &args.domain,
        QueryType::A,
    );
    socket
        .send(&serialize_to_bytes(&query))
        .expect("send message fail");

    let mut buf = [0; 1024];
    let (number_of_buf, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

    println!("{:?}", String::from_utf8(Vec::from(&buf[..number_of_buf])));
    Ok(())
}
