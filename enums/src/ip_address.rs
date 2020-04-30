use std::fmt;
use std::fmt::Formatter;
use std::net::{Ipv4Addr, Ipv6Addr};

enum IpAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

impl IpAddress {

    fn info(&self) -> (&str, String) {
        match self {
            IpAddress::V4(address) => ("v4", address.to_string()),
            IpAddress::V6(address) => ("v6", address.to_string())
        }
    }

    fn ip_version(&self) -> &str {
        self.info().0
    }

    fn address(&self) -> String {
        self.info().1
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} via IP {}", self.address(), self.ip_version())
    }
}

pub fn demo_ip_address() {
    let home = IpAddress::V4(
        Ipv4Addr::new(127, 0, 0, 1)
    );

    let loopback = IpAddress::V6(
        Ipv6Addr::new(0, 0, 0 , 0 , 0 , 0, 0 , 1)
    );

    println!("{}", home);
    println!("{}", loopback);
}
