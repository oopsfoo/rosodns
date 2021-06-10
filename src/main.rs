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

    #[structopt(long = "listen_ip", default_value = "127.0.0.1")]
    listen_ip: Ipv4Addr,

    #[structopt(long ="listen_port", default_value = "11223")]
    listen_port: u16,

    #[structopt(long = "socks5_ip", default_value = "127.0.0.1")]
    socks5_ip: Ipv4Addr,

    #[structopt(long ="socks5_port", default_value = "7890")]
    socks5_port: u16,

    #[structopt(long = "dns_ip", default_value = "8.8.8.8")]
    dns_ip: Ipv4Addr,

    #[structopt(long ="dns_port", default_value = "53")]
    dns_port: u16,
}

struct Socks5Header {
    header: [u8; 4],
    ip: [u8; 4],
    port: [u8; 2],
}

fn get_socks5_header() -> Vec<u8> {
    let opt = Opt::from_args();
    let dns_ip = opt.dns_ip;
    let dns_bytes = dns_ip.octets();
    let port_value = opt.dns_port;
    let port_bytes = port_value.to_be_bytes();
    let socks5_header = Socks5Header{
        header: [0x00, 0x00, 0x00, 0x01],
        ip: dns_bytes,
        port: port_bytes,
    };
    let mut full_header = Vec::new();
    full_header.extend(socks5_header.header.iter().copied());
    full_header.extend(socks5_header.ip.iter().copied());
    full_header.extend(socks5_header.port.iter().copied());
    println!("----------------- build socks5 header --------------");
    assert_eq!(pretty_hex(&full_header), format!("{:?}", full_header.hex_dump()));
    println!("{:?}", full_header.hex_dump());
    return full_header
}

fn get_listen_addr() -> String {
    let opt = Opt::from_args();
    return format!("{}:{}", opt.listen_ip, opt.listen_port);
}

fn get_dns_addr() -> String {
    let opt = Opt::from_args();
    return format!("{}:{}", opt.dns_ip, opt.dns_port);
}

fn get_socks5_addr() -> String {
    let opt = Opt::from_args();
    return format!("{}:{}", opt.socks5_ip, opt.socks5_port);
}

fn repack_socks5_udp(mut data:Vec<u8>) -> Vec<u8> {
    let mut pkt = vec![]  ;
    pkt.append(&mut get_socks5_header());
    pkt.append(&mut data);
    println!("----------------- build socks5 pkt --------------");
    assert_eq!(pretty_hex(&pkt), format!("{:?}", pkt.hex_dump()));
    println!("{:?}", pkt.hex_dump());
    return pkt
}

fn request_socks5_svr() {

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sock = UdpSocket::bind(get_listen_addr()).await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let pkt = repack_socks5_udp(buf[0..len].to_vec());
        let client = UdpSocket::bind("127.0.0.1:12345").await?;
        client.connect(get_socks5_addr()).await?;
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
