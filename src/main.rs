use std::error::Error;
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use pretty_hex::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short = "h", long = "host", default_value = "8.8.8.8")]
    host: Ipv4Addr,

    #[structopt(short = "p", long = "port", default_value = "53")]
    port: u16,
}

struct Socks5Header {
    header: [u8; 4],
    ip: [u8; 4],
    port: [u8; 2],
}

fn socks5Header() -> Vec<u8> {
    let opt = Opt::from_args();
    let dns_ip = opt.host;
    let dns_bytes = dns_ip.octets();
    let port_value = opt.port;
    let port_bytes = port_value.to_be_bytes();
    let socks5Header = Socks5Header{
        header: [0x00, 0x00, 0x00, 0x01],
        ip: dns_bytes,
        port: port_bytes,
    };
    let mut full_header = Vec::new();
    full_header.extend(socks5Header.header.iter().copied());
    full_header.extend(socks5Header.ip.iter().copied());
    full_header.extend(socks5Header.port.iter().copied());
    assert_eq!(pretty_hex(&full_header), format!("{:?}", full_header.hex_dump()));
    println!("{:?}", full_header.hex_dump());
    return full_header
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socks5_header = socks5Header();
    let sock = UdpSocket::bind("127.0.0.1:11223").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let v = &buf[0..len];
        assert_eq!(pretty_hex(&v), format!("{:?}", v.hex_dump()));
        println!("{:?}", v.hex_dump());
        let mut pkt = vec![]  ;
        pkt.append(&mut socks5_header);
        pkt.append(&mut v.to_vec());

        println!("----------------- request to socks server --------------");
        assert_eq!(pretty_hex(&pkt), format!("{:?}", pkt.hex_dump()));
        println!("{:?}", pkt.hex_dump());

        let client = UdpSocket::bind("127.0.0.1:12345").await?;
        client.connect("127.0.0.1:7890").await?;
        client.send(&pkt).await?;
        let mut rsp = vec![0u8; 1024];
        let rsp_len = client.recv(&mut rsp).await?;
        println!("----------------- response from socks server --------------");
        assert_eq!(pretty_hex(&rsp), format!("{:?}", rsp.hex_dump()));
        println!("{:?}", rsp.hex_dump());

        println!("----------------- start ret --------------");
        let ret = &rsp[10..];
        assert_eq!(pretty_hex(&ret), format!("{:?}", ret.hex_dump()));
        println!("{:?}", ret.hex_dump());

        let len = sock.send_to(ret, addr).await?;



        println!("{:?} bytes sent", len);
    }
}
