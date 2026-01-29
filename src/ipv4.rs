pub struct IPv4Address {
    pub address: (u8, u8, u8, u8),
    pub cidr_bits: u8,
    pub subnet_mask: (u8, u8, u8, u8),
}

impl IPv4Address {
    pub fn parse_ip(input: &str) -> Option<IPv4Address> {
        let addr_subnet_mask_vec: Vec<&str> = input.split('/').collect();
        if addr_subnet_mask_vec.len() != 2 {
            return None;
        }

        let cidr_bits: &str = addr_subnet_mask_vec[1];
        let octets: Vec<&str> = addr_subnet_mask_vec[0].split('.').collect();
        if octets.len() != 4 {
            return None;
        }

        let mut address = IPv4Address {
            address: (0, 0, 0, 0),
            cidr_bits: 0,
            subnet_mask: (0, 0, 0, 0),
        };

        // Calling unwrap should be okay as we are working with a
        // vector that has values from index zero through three.
        address.address.0 = match octets.get(0).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return None,
        };
        address.address.1 = match octets.get(1).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return None,
        };
        address.address.2 = match octets.get(2).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return None,
        };
        address.address.3 = match octets.get(3).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return None,
        };

        // There are only 32 bits in four octets, therefore the
        // ipv4 subnet bask should only be up until 32 bits.
        address.cidr_bits = match cidr_bits.trim_end().parse() {
            Ok(value) if value <= 32 => value,
            _ => return None,
        };

        let mut mask = if address.cidr_bits == 0 {
            0
        } else {
            (!0u32) << (32 - address.cidr_bits)
        };
        address.subnet_mask.3 = mask as u8 & 0xFF;

        mask = mask >> 8;
        address.subnet_mask.2 = mask as u8 & 0xFF;

        mask = mask >> 8;
        address.subnet_mask.1 = mask as u8 & 0xFF;

        mask = mask >> 8;
        address.subnet_mask.0 = mask as u8 & 0xFF;

        Some(address)
    }
}

#[cfg(test)]
mod tests {
    use super::IPv4Address;

    #[test]
    fn subnet_mask_parsing() {
        let addr = IPv4Address::parse_ip("1.2.1.3/24");
        assert!(addr.is_some());
        assert_eq!(addr.unwrap().subnet_mask, (255, 255, 255, 0));

        let addr = IPv4Address::parse_ip("1.2.1.3/15");
        assert!(addr.is_some());
        assert_eq!(addr.unwrap().subnet_mask, (255, 254, 0, 0));

        let addr = IPv4Address::parse_ip("1.2.1.3/27");
        assert!(addr.is_some());
        assert_eq!(addr.unwrap().subnet_mask, (255, 255, 255, 224));
    }

    #[test]
    fn ip_is_parsed_only_if_octets_are_valid() {
        assert!(IPv4Address::parse_ip("256.255.255.1/23").is_none());
        assert!(IPv4Address::parse_ip("255.257.255.1/23").is_none());
        assert!(IPv4Address::parse_ip("255.255.259.1/23").is_none());
        assert!(IPv4Address::parse_ip("255.255.255.258/23").is_none());
        assert!(IPv4Address::parse_ip("255.255.255.255/23").is_some());
    }

    #[test]
    fn ip_is_parsed_only_if_subnet_mask_is_valid() {
        assert!(IPv4Address::parse_ip("1.1.3.1/33").is_none());
        assert!(IPv4Address::parse_ip("1.1.1.1/32").is_some());
        assert!(IPv4Address::parse_ip("1.1.1.1/0").is_some());
    }

    #[test]
    fn ip_is_parsed_only_if_addr_subnet_mask_is_present() {
        assert!(IPv4Address::parse_ip("1.12.1.4").is_none());
        assert!(IPv4Address::parse_ip("1.3.1.5/23").is_some());
    }

    #[test]
    fn ip_is_parsed_only_if_there_are_4_octets() {
        assert!(IPv4Address::parse_ip("1.1.1/12").is_none());
        assert!(IPv4Address::parse_ip("1.1.1.1.1/12").is_none());
        assert!(IPv4Address::parse_ip("1.1.1.1/24").is_some());
    }
}
