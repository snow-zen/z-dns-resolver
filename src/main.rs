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

/// 制作 question 头
fn make_question_header(
    query_id: u16,
    flag: u16,
    num_questions: u16,
    num_answers: u16,
    num_auth: u16,
    num_additional: u16,
) -> Vec<u8> {
    [
        query_id.to_be_bytes(),
        flag.to_be_bytes(),
        num_questions.to_be_bytes(),
        num_answers.to_be_bytes(),
        num_auth.to_be_bytes(),
        num_additional.to_be_bytes(),
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use crate::make_question_header;

    fn to_hex_str(bytes: Vec<u8>) -> String {
        let mut result = String::new();
        for byte in bytes {
            result.push_str(format!("{:02x}", byte).as_str())
        }
        result
    }

    #[test]
    fn test_make_question_header() {
        let question_header = make_question_header(0xb962, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000);
        assert_eq!(question_header.len(), 12);
        assert_eq!(to_hex_str(question_header), "b96201000001000000000000");
    }
}
