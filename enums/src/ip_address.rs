use std::fmt;
use std::fmt::Formatter;


#[derive(Debug)]
enum IpAddressVersion {
    V4,
    V6
}

impl fmt::Display for IpAddressVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn route(ip_version: &IpAddressVersion) {
    println!("Routing via {} IP {}!",
             match ip_version {
                 IpAddressVersion::V6 => "new and improved",
                 _ => "perfectly acceptable"
             },
             ip_version.to_string().to_ascii_lowercase()
    )
}

pub fn demo_ip_address() {
    for v in [IpAddressVersion::V4, IpAddressVersion::V6].iter() {
        println!("{:?}", v);
        route(v);
    }
}
