use std::env;
use subnetinfo::ipv4::IPv4Address;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut empty_args = true;

    for arg in argv.iter() {
        let ipv4_parse_res = arg.parse::<IPv4Address>();
        if let Ok(addr) = ipv4_parse_res {
            empty_args = false;
            println!("{}", addr.info())
        }
    }

    if empty_args {
        println!("Usage: subnetinfo [IP Addresses]")
    }
}
