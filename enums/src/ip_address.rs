use std::fmt;
use std::fmt::Formatter;


#[derive(Debug)]
enum IpAddressVersion {
    V4,
    V6,
}

impl fmt::Display for IpAddressVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct IpAddress {
    version: IpAddressVersion,
    address: String,
}

impl IpAddress {
    fn version_name(&self) -> String {
        format!("IP {}", self.version.to_string().to_ascii_lowercase())
    }
}

fn route(ip_address: &IpAddress) {
    println!(
        "Routing to {} via {} {}!",
        ip_address.address,
        match ip_address.version {
            IpAddressVersion::V6 => "new and improved",
            _ => "perfectly acceptable"
        },
        ip_address.version_name()
    )
}

pub fn demo_ip_address() {
    let home = IpAddress {
        version: IpAddressVersion::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        version: IpAddressVersion::V6,
        address: String::from("::1"),
    };

    route(&home);
    route(&loopback);
}
