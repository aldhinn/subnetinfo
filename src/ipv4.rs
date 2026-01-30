use std::{io::ErrorKind, str::FromStr};

pub struct IPv4Address {
    address: (u8, u8, u8, u8),
    cidr_bits: u8,
    subnet_mask: (u8, u8, u8, u8),
    network_address: (u8, u8, u8, u8),
    broadcast_address: (u8, u8, u8, u8),
}

impl FromStr for IPv4Address {
    type Err = ErrorKind;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let addr_subnet_mask_vec: Vec<&str> = input.split('/').collect();
        if addr_subnet_mask_vec.len() != 2 {
            return Err(ErrorKind::InvalidData);
        }

        let cidr_bits: &str = addr_subnet_mask_vec[1];
        let octets: Vec<&str> = addr_subnet_mask_vec[0].split('.').collect();
        if octets.len() != 4 {
            return Err(ErrorKind::InvalidData);
        }

        let mut address = IPv4Address {
            address: (0, 0, 0, 0),
            cidr_bits: 0,
            subnet_mask: (0, 0, 0, 0),
            network_address: (0, 0, 0, 0),
            broadcast_address: (0, 0, 0, 0),
        };

        // Calling unwrap should be okay as we are working with a
        // vector that has values from index zero through three.
        address.address.0 = match octets.get(0).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return Err(ErrorKind::InvalidData),
        };
        address.address.1 = match octets.get(1).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return Err(ErrorKind::InvalidData),
        };
        address.address.2 = match octets.get(2).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return Err(ErrorKind::InvalidData),
        };
        address.address.3 = match octets.get(3).unwrap().trim_end().parse() {
            Ok(value) => value,
            _ => return Err(ErrorKind::InvalidData),
        };

        // There are only 32 bits in four octets, therefore the
        // ipv4 subnet bask should only be up until 32 bits.
        address.cidr_bits = match cidr_bits.trim_end().parse() {
            Ok(value) if value <= 32 => value,
            _ => return Err(ErrorKind::InvalidData),
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

        address.network_address = (
            address.address.0 & address.subnet_mask.0,
            address.address.1 & address.subnet_mask.1,
            address.address.2 & address.subnet_mask.2,
            address.address.3 & address.subnet_mask.3,
        );

        address.broadcast_address = (
            address.network_address.0 | !address.subnet_mask.0,
            address.network_address.1 | !address.subnet_mask.1,
            address.network_address.2 | !address.subnet_mask.2,
            address.network_address.3 | !address.subnet_mask.3,
        );

        Ok(address)
    }
}

impl IPv4Address {
    pub fn info(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}",
            format!(
                "=====================================\nHost: {}.{}.{}.{}/{}",
                self.address.0, self.address.1, self.address.2, self.address.3, self.cidr_bits,
            ),
            format!(
                "=====================================\nIP Address: {}.{}.{}.{}",
                self.address.0, self.address.1, self.address.2, self.address.3
            ),
            format!(
                "Subnet Mask: {}.{}.{}.{}",
                self.subnet_mask.0, self.subnet_mask.1, self.subnet_mask.2, self.subnet_mask.3
            ),
            format!(
                "Network Address: {}.{}.{}.{}",
                self.network_address.0,
                self.network_address.1,
                self.network_address.2,
                self.network_address.3,
            ),
            format!(
                "Broadcast Address: {}.{}.{}.{}",
                self.broadcast_address.0,
                self.broadcast_address.1,
                self.broadcast_address.2,
                self.broadcast_address.3,
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::IPv4Address;

    #[test]
    fn subnet_mask_parsing() {
        let addr = "1.2.1.3/24".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().subnet_mask, (255, 255, 255, 0));

        let addr = "1.2.1.3/15".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().subnet_mask, (255, 254, 0, 0));

        let addr = "1.2.1.3/27".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().subnet_mask, (255, 255, 255, 224));

        let addr = "1.2.1.3/0".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().subnet_mask, (0, 0, 0, 0));
    }

    #[test]
    fn network_address_calculation() {
        let addr = "1.2.1.3/28".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().network_address, (1, 2, 1, 0));

        let addr = "1.2.1.15/28".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().network_address, (1, 2, 1, 0));

        let addr = "1.2.1.31/28".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().network_address, (1, 2, 1, 16));
    }

    #[test]
    fn broadcast_address_calculation() {
        let addr = "1.2.1.3/28".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().broadcast_address, (1, 2, 1, 15));

        let addr = "1.2.1.16/28".parse::<IPv4Address>();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().broadcast_address, (1, 2, 1, 31));
    }

    #[test]
    fn ip_is_parsed_only_if_octets_are_valid() {
        assert!("256.255.255.1/23".parse::<IPv4Address>().is_err());
        assert!("255.257.255.1/23".parse::<IPv4Address>().is_err());
        assert!("255.255.259.1/23".parse::<IPv4Address>().is_err());
        assert!("255.255.255.258/23".parse::<IPv4Address>().is_err());
        assert!("255.255.255.255/23".parse::<IPv4Address>().is_ok());
    }

    #[test]
    fn ip_is_parsed_only_if_subnet_mask_is_valid() {
        assert!("1.1.3.1/33".parse::<IPv4Address>().is_err());
        assert!("1.1.1.1/32".parse::<IPv4Address>().is_ok());
        assert!("1.1.1.1/0".parse::<IPv4Address>().is_ok());
    }

    #[test]
    fn ip_is_parsed_only_if_addr_subnet_mask_is_present() {
        assert!("1.12.1.4".parse::<IPv4Address>().is_err());
        assert!("1.3.1.5/23".parse::<IPv4Address>().is_ok());
    }

    #[test]
    fn ip_is_parsed_only_if_there_are_4_octets() {
        assert!("1.1.1/12".parse::<IPv4Address>().is_err());
        assert!("1.1.1.1.1/12".parse::<IPv4Address>().is_err());
        assert!("1.1.1.1/24".parse::<IPv4Address>().is_ok());
    }
}
