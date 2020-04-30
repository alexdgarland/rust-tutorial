
#[derive(Debug)]
enum IpAddressVersion {
    V4,
    V6
}

fn route(ip_version: &IpAddressVersion) {
    println!(
        "Routing via {}!",
        match ip_version {
            IpAddressVersion::V4 => "IP v4",
            IpAddressVersion::V6 => "new and improved IP v6"
        }
    )
}

pub fn demo_ip_address() {
    for v in [IpAddressVersion::V4, IpAddressVersion::V6].iter() {
        println!("{:?}", v);
        route(v);
    }
}
