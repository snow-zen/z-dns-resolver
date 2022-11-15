mod query;

use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    /// Specify domain
    #[arg(short, long)]
    domain: String,
}

const DNS_PROTOCOL_DEFAULT_PORT: u16 = 53;

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

    socket
        .connect((
            Ipv4Addr::from_str("114.114.114.114").unwrap(),
            DNS_PROTOCOL_DEFAULT_PORT,
        ))
        .expect("connect fail");

    socket
        .send(&make_dns_query(args.domain.as_str(), 1)[..])
        .expect("send message fail");

    let mut buf = [0; 1024];
    let (number_of_buf, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

    println!("{:?}", String::from_utf8(Vec::from(&buf[..number_of_buf])));
    Ok(())
}

/// 制作 header 部分
fn make_header(
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

/// 制作 question 部分
fn make_question(domain: &str, query_type: u16) -> Vec<u8> {
    let query_class: u16 = 1;
    vec![
        encode_domain(domain),
        Vec::from(query_type.to_be_bytes()),
        Vec::from(query_class.to_be_bytes()),
    ]
    .concat()
}

/// 制作 DNS 查询请求数据
fn make_dns_query(domain: &str, query_type: u16) -> Vec<u8> {
    let query_id = rand::thread_rng().gen_range(0..65535);
    let header = make_header(query_id, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000);
    let question = make_question(domain, query_type);
    [header, question].concat()
}

/// 编码 domain
fn encode_domain(domain: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for part in domain.split('.') {
        result.extend(
            u8::try_from(part.len())
                .expect("domain part is too long")
                .to_be_bytes(),
        );
        result.extend(part.as_bytes())
    }
    result.extend("\0".as_bytes());
    return result;
}

#[cfg(test)]
mod tests {
    use crate::{encode_domain, make_header, make_question};

    fn to_hex_str(bytes: Vec<u8>) -> String {
        let mut result = String::new();
        for byte in bytes {
            result.push_str(format!("{:02x}", byte).as_str())
        }
        result
    }

    #[test]
    fn test_make_header() {
        let question_header = make_header(0xb962, 0x0100, 0x0001, 0x0000, 0x0000, 0x0000);
        assert_eq!(question_header.len(), 12);
        assert_eq!(to_hex_str(question_header), "b96201000001000000000000");
    }

    #[test]
    fn test_encode_domain() {
        let encode_str = encode_domain("example.com");
        assert_eq!(to_hex_str(encode_str), "076578616d706c6503636f6d00");
    }

    #[test]
    fn test_question_header() {
        let question = make_question("example.com", 1);
        assert_eq!(to_hex_str(question), "076578616d706c6503636f6d0000010001")
    }
}
