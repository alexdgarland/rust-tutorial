use super::implementation::IpAddress;
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn demo_ip_address() {
    let home = IpAddress::V4(
        Ipv4Addr::new(127, 0, 0, 1)
    );

    let loopback = IpAddress::V6(
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
    );

    println!("{}", home);
    println!("{}", loopback);
}
