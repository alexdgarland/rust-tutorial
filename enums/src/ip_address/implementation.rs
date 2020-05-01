use std::fmt;
use std::fmt::Formatter;
use std::net::{Ipv4Addr, Ipv6Addr};

pub enum IpAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
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
