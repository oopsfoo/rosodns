use std::error::Error;
use tokio::net::UdpSocket;
use pretty_hex::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socks5_header = vec![0x00, 0x00, 0x00, 0x01];
    assert_eq!(pretty_hex(&socks5_header), format!("{:?}", socks5_header.hex_dump()));
    println!("{:?}", socks5_header.hex_dump());
    let mut socks5_ip = vec![8, 8, 8, 8];
    assert_eq!(pretty_hex(&socks5_ip), format!("{:?}", socks5_ip.hex_dump()));
    println!("{:?}", socks5_ip.hex_dump());
    let mut socks5_port = vec![0, 53];
    assert_eq!(pretty_hex(&socks5_port), format!("{:?}", socks5_port.hex_dump()));
    println!("{:?}", socks5_port.hex_dump());

    let sock = UdpSocket::bind("127.0.0.1:11223").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let v = &buf[0..len];
        assert_eq!(pretty_hex(&v), format!("{:?}", v.hex_dump()));
        println!("{:?}", v.hex_dump());
        let mut pkt = vec![]  ;
        pkt.append(&mut socks5_header);
        pkt.append(&mut socks5_ip);
        pkt.append(&mut socks5_port);
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
