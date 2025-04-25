use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() {
    // placeholder!
    let args: Vec<String> = std::env::args().collect();
    
    let (translator_id, addr_str) = match (args.get(1), args.get(2)) {
        (None, None) => (7, "192.168.1.0".to_string()),
        (Some(addr_str), None) => (7, addr_str.to_owned()),
        (Some(translator_id), Some(addr_str)) => (
            translator_id.parse::<u32>().expect("Invalid translator ID"),
            addr_str.to_owned(),
        ),
        _ => panic!("something very strange happened with numbers")
    };
    let addr = Ipv4Addr::from_str(&addr_str).expect("Invalid IPv4 address");
    
    
    // according to rust spec, narrowing conversions truncate. no need to mask.
    let mut prefix: [u8; 16] = [0xfd, 0x7a, 0x11, 0x5c, 0xa1, 0xe0, 0xb, 0x1a,
        0, 0, 0, 0, 0, 0, 0, 0];
    prefix[8..12].copy_from_slice(&translator_id.to_be_bytes());
    prefix[12..16].copy_from_slice(&addr.octets());
    
    println!("{}", Ipv6Addr::from(prefix))
}