mod query;

use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

use crate::query::{serialize_to_bytes, Query, QueryType};
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
