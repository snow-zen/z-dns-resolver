use clap::Parser;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    /// Specify domain
    #[arg(short, long)]
    domain: String,
}

const DNS_PROTOCOL_DEFAULT_PORT: u16 = 53;

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    println!("parsing {}...", args.domain);
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    println!("local addr: {}", socket.local_addr().unwrap());

    let mut buf = [0; 32];
    let result = socket.connect((
        Ipv4Addr::from_str("114.114.114.114").unwrap(),
        DNS_PROTOCOL_DEFAULT_PORT,
    ));
    Ok(())
}