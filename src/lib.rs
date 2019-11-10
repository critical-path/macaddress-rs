/// # The `macaddress` module
///
/// This module contains one struct, `MediaAccessControlAddress`, with
/// which you interact directly.  Using it is easy!
///
/// ```
/// /// Bring the `MediaAccessControlAddress` struct into scope in `main.rs`
/// /// `lib.rs`, or some other relevant file.
///
/// use macaddress::macaddress::MediaAccessControlAddress;
///
/// /// Instantiate `MediaAccessControlAddress` by calling the `new`
/// /// method and passing in a MAC address in plain, hyphen, colon, or dot 
/// /// notation.
///
/// /// Plain notation:
///
/// let digits = String::from("a0b1c2d3e4f5");
/// let mac = MediaAccessControlAddress::new(&digits).unwrap();
///
/// /// Hyphen notation:
///
/// let digits = String::from("a0-b1-c2-d3-e4-f5");
/// let mac = MediaAccessControlAddress::new(&digits).unwrap();
///
/// /// Colon notation:
///
/// let digits = String::from("a0:b1:c2:d3:e4:f5");
/// let mac = MediaAccessControlAddress::new(&digits).unwrap();
///
/// /// Dot notation:
///
/// let digits = String::from("a0b1.c2d3.e4f5");
/// let mac = MediaAccessControlAddress::new(&digits).unwrap();
///
/// /// Call one or more of `MediaAccessControlAddress`'s methods.
///
/// let broadcast = mac.is_broadcast();
/// println!("{}", &broadcast);
///
/// let multicast = mac.is_multicast();
/// println!("{}", &multicast);
///
/// let unicast = mac.is_unicast();
/// println!("{}", &unicast);
///
/// let uaa = mac.is_uaa();
/// println!("{}", &uaa);
///
/// let laa = mac.is_laa();
/// println!("{}", &laa);
///
/// let kind = mac.kind();
/// println!("{}", &kind);
///
/// let oui = mac.has_oui();
/// println!("{}", &oui);
///
/// let cid = mac.has_cid();
/// println!("{}", &cid);
///
/// let binary = mac.to_binary_representation();
/// println!("{}", &binary);
///
/// let plain = mac.to_plain_notation();
/// println!("{}", &plain);
///
/// let hyphen = mac.to_hyphen_notation();
/// println!("{}", &hyphen);
///
/// let colon = mac.to_colon_notation();
/// println!("{}", &colon);
///
/// let dot = mac.to_dot_notation();
/// println!("{}", &dot);
///
/// let fragments = mac.to_fragments();
/// println!("{:?}", &fragments);
/// ```
pub mod macaddress {
    use super::utils;

    /// `MediaAccessControlAddress` makes it easy to work with
    /// media access control (MAC) addresses.
    ///
    /// MAC addresses have properties of their own, but they also
    /// inherit/share the properties of 48-bit IEEE extended identifiers.
    ///
    /// Extended identifiers are either extended unique identifiers
    /// (EUI) or extended local identifiers (ELI).  EUIs have
    /// organizationally-unique identifiers (OUO), while ELIs have
    /// company IDs (CID).
    ///
    /// For more information, visit the following URL:
    /// <https://standards.ieee.org/products-services/regauth/tut/index.html>.
    #[derive(Debug)]
    pub struct MediaAccessControlAddress {
        value: String,
    }

    impl MediaAccessControlAddress {
        /// Instantiates `MediaAccessControlAddress` with
        /// 12 hexadecimal digits (`0-9`, `A-F`, or `a-f`) in
        /// plain, hyphen, colon, or dot notation.
        pub fn new(digits: &str) -> Result<Self, String> {
            if utils::NOTATIONS.is_match(&digits) {
                let address = utils::clean(&digits);
                Ok(Self { value: address })
            } else {
                Err(String::from("Pass in 12 hexadecimal digits."))
            }
        }

        /// Returns the binary representation of the MAC address.
        /// *The most-significant digit of each octet appears first.*
        pub fn to_binary_representation(&self) -> String {
            let binary: Vec<String> = utils::TWO_DIGITS
                .find_iter(&self.value)
                .map(|element| {
                    let element = element.as_str();
                    let decimal = usize::from_str_radix(&element, 16).unwrap();
                    format!("{:08b}", &decimal)
                })
                .collect();

            binary.join("")
        }

        /// Returns the decimal representation of the MAC address.
        pub fn to_decimal_representation(&self) -> usize {
            let binary = self.to_binary_representation();
            usize::from_str_radix(&binary, 2).unwrap()   
        }

        /// Returns the MAC address in plain notation
        /// (for example, `a0b1c2d3e4f5`).
        pub fn to_plain_notation(&self) -> String {
            self.value.to_string()
        }

        /// Returns the MAC address in hyphen notation
        /// (for example, `a0-b1-c2-d3-e4-f5`).
        pub fn to_hyphen_notation(&self) -> String {
            let hyphen: Vec<&str> = utils::TWO_DIGITS
                .find_iter(&self.value)
                .map(|element| element.as_str())
                .collect();

            hyphen.join("-")
        }

        /// Returns the MAC address in colon notation
        /// (for example, `a0:b1:c2:d3:e4:f5`).
        pub fn to_colon_notation(&self) -> String {
            let colon: Vec<&str> = utils::TWO_DIGITS
                .find_iter(&self.value)
                .map(|element| element.as_str())
                .collect();

            colon.join(":")
        }

        /// Returns the MAC address in dot notation
        /// (for example, `a0b1.c2d3.e4f5`).
        pub fn to_dot_notation(&self) -> String {
            let dot: Vec<&str> = utils::FOUR_DIGITS
                .find_iter(&self.value)
                .map(|element| element.as_str())
                .collect();

            dot.join(".")
        }

        /// Returns the MAC address's two "fragments,"
        /// where the first 24 bits are an OUI or CID and
        /// the second 24 bits are specific to an interface
        /// (for example, `(a0b1c2, d3e4f5)`.
        pub fn to_fragments(&self) -> (&str, &str) {
            let (first, second) = &self.value.split_at(6);
            (first, second)
        }

        /// Returns the MAC address's kind, where kind is
        /// `unique`, `local`, or `unknown`.
        ///
        /// The two least-significant bits in the first octet
        /// of a MAC address/extended identifier determine
        /// whether it is an EUI (`00` = `unique`).
        ///
        /// The four least-significant bits in the first octet
        /// of a MAC address/extended identifier determine
        /// whether it is an ELI (`1010` = `local`).
        pub fn kind(&self) -> String {
            let binary = self.to_binary_representation();

            if &binary[6..8] == "00" {
                String::from("unique")
            } else if &binary[4..8] == "1010" {
                String::from("local")
            } else {
                String::from("unknown")
            }
        }

        /// Whether the MAC address/extended identifier has
        /// an OUI.
        ///
        /// If the MAC address/exended identifier is an EUI,
        /// then it has an OUI.
        pub fn has_oui(&self) -> bool {
            self.kind() == "unique"
        }

        /// Whether the MAC address/extended identifier has
        /// a CID.
        ///
        /// If the MAC address/extended identifier is an ELI,
        /// then it has a CID.
        pub fn has_cid(&self) -> bool {
            self.kind() == "local"
        }

        /// Whether the MAC address is a broadcast address
        /// (`ffffffffffff` = broadcast).
        pub fn is_broadcast(&self) -> bool {
            let address = &self.value;
            address == "ffffffffffff"
        }

        /// Whether the MAC address is a multicast address
        /// (layer-two multicast, not layer-three multicast).
        ///
        /// The least-significant bit in the first octet of
        /// a MAC address determines whether it is a multicast
        /// or a unicast (`1` = multicast).
        pub fn is_multicast(&self) -> bool {
            let binary = self.to_binary_representation();
            &binary[7..8] == "1"
        }

        /// Whether the MAC address is a unicast address.
        ///
        /// The least-significant bit in the first octet of
        /// a MAC address determines whether it is a multicast
        /// or a unicast (`0` = unicast).
        pub fn is_unicast(&self) -> bool {
            !self.is_multicast()
        }

        /// Whether the MAC address is a universally-administered
        /// address (UAA).
        ///
        /// The second-least-significant bit in the first octet of
        /// a MAC address determines whether it is a UAA or an LAA
        /// (`0` = UAA).
        pub fn is_uaa(&self) -> bool {
            let binary = self.to_binary_representation();
            self.is_unicast() && &binary[6..7] == "0"
        }

        /// Whether the MAC address is a locally-administered
        /// address (LAA).
        ///
        /// The second-least-significant bit in the first octet of
        /// a MAC address determines whether it is a UAA or an LAA
        /// (`1` = LAA).
        pub fn is_laa(&self) -> bool {
            let binary = self.to_binary_representation();
            self.is_unicast() && &binary[6..7] == "1"
        }
    }
}

/// # The `utils` module
///
/// This module contains macros and functions required by the
/// `macaddress` module.
pub mod utils {
    use lazy_static::lazy_static;
    use regex::{Regex, RegexSet};

    lazy_static! {
        /// These patterns represent a MAC address in plain,
        /// hyphen, colon, or dot notation.
        pub static ref NOTATIONS: RegexSet = RegexSet::new(&[
            "^[0-9A-Fa-f]{12}$",
            "^([0-9A-Fa-f]{2}[-]{1}){5}[0-9A-Fa-f]{2}$",
            "^([0-9A-Fa-f]{2}[:]{1}){5}[0-9A-Fa-f]{2}$",
            "^([0-9A-Fa-f]{4}[.]{1}){2}[0-9A-Fa-f]{4}$"
        ])
        .unwrap();

        /// This pattern represents any character that is not a
        /// hexadecimal digit.
        pub static ref NOT_DIGITS: Regex = Regex::new("[^0-9A-Fa-f]").unwrap();

        /// This pattern represents a series of two hexadecimal
        /// digits.
        pub static ref TWO_DIGITS: Regex = Regex::new("[0-9a-f]{2}").unwrap();

        /// This pattern represents a series of four hexadecimal
        /// digits.
        pub static ref FOUR_DIGITS: Regex = Regex::new("[0-9a-f]{4}").unwrap();
    }

    /// "Cleans" a MAC address by converting uppercase to lowercase 
    /// letters and removing all hyphens, colons, and dots.
    pub fn clean(digits: &str) -> String {
        let lowercase = &digits.to_lowercase();
        let clean = NOT_DIGITS.replace_all(&lowercase, "");
        clean.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::macaddress::MediaAccessControlAddress;

    #[test]
    #[should_panic]
    fn test_invalid_addresses() {
        let addresses = [
            "0a",                 // Too few digits
            "0a1b2c3d4e5f6",      // Too many digits
            "0a1b2c3d4e5g",       // Invalid digit
            "-0a-1b-2c-3d-4e-5f", // Leading hyphen
            "0a-1b-2c-3d-4e-5f-", // Trailing hyphen
            "0a-1b-2c-3d-4e5f",   // Missing hyphen
            ":0a:1b:2c:3d:4e:5f", // Leading colon
            "0a:1b:2c:3d:4e:5f:", // Trailing colon
            "0a:1b:2c:3d:4e5f",   // Missing colon
            ".0a1b.2c3d.4e5f",    // Leading dot
            "0a1b.2c3d.4e5f.",    // Trailing dot
            "0a1b.2c3d4e5f",      // Missing dot
        ];

        for element in addresses.into_iter() {
            let digits = element.to_string();
            MediaAccessControlAddress::new(&digits).unwrap();
        }
    }

    // An EUI is a unicast address.
    #[test]
    fn test_unicast_eui_addresses() {
        let addresses = [
            (
                "a0b1c2d3e4f5", // Plain notation (lowercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "A0B1C2D3E4F5", // Plain notation (uppercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "a0-b1-c2-d3-e4-f5", // Hyphen notation (lowercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "A0-B1-C2-D3-E4-F5", // Hyphen notation (uppercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "a0:b1:c2:d3:e4:f5", // Colon notation (lowercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "A0:B1:C2:D3:E4:F5", // Colon notation (uppercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "a0b1.c2d3.e4f5", // Dot notation (lowercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
            (
                "A0B1.C2D3.E4F5", // Dot notation (uppercase)
                "101000001011000111000010110100111110010011110101",
                176685338322165,
                "a0b1c2d3e4f5",
                "a0-b1-c2-d3-e4-f5",
                "a0:b1:c2:d3:e4:f5",
                "a0b1.c2d3.e4f5",
                ("a0b1c2", "d3e4f5"),
                "unique",
                true,
                false,
                false,
                false,
                true,
                true,
                false,
            ),
        ];

        for element in addresses.into_iter() {
            let digits = element.0.to_string();
            let mac = MediaAccessControlAddress::new(&digits).unwrap();

            assert_eq!(mac.to_binary_representation(), element.1);
            assert_eq!(mac.to_decimal_representation(), element.2);
            assert_eq!(mac.to_plain_notation(), element.3);
            assert_eq!(mac.to_hyphen_notation(), element.4);
            assert_eq!(mac.to_colon_notation(), element.5);
            assert_eq!(mac.to_dot_notation(), element.6);

            assert_eq!(mac.to_fragments(), element.7);
            assert_eq!(mac.kind(), element.8);
            assert_eq!(mac.has_oui(), element.9);
            assert_eq!(mac.has_cid(), element.10);

            assert_eq!(mac.is_broadcast(), element.11);
            assert_eq!(mac.is_multicast(), element.12);
            assert_eq!(mac.is_unicast(), element.13);
            assert_eq!(mac.is_uaa(), element.14);
            assert_eq!(mac.is_laa(), element.15);
        }
    }

    // An ELI is a unicast address.
    #[test]
    fn test_unicast_eli_address() {
        let address = (
            "0a1b2c3d4e5f",
            "000010100001101100101100001111010100111001011111",
            11111822610015,
            "0a1b2c3d4e5f",
            "0a-1b-2c-3d-4e-5f",
            "0a:1b:2c:3d:4e:5f",
            "0a1b.2c3d.4e5f",
            ("0a1b2c", "3d4e5f"),
            "local",
            false,
            true,
            false,
            false,
            true,
            false,
            true,
        );

        let digits = address.0.to_string();
        let mac = MediaAccessControlAddress::new(&digits).unwrap();

        assert_eq!(mac.to_binary_representation(), address.1);
        assert_eq!(mac.to_decimal_representation(), address.2);
        assert_eq!(mac.to_plain_notation(), address.3);
        assert_eq!(mac.to_hyphen_notation(), address.4);
        assert_eq!(mac.to_colon_notation(), address.5);
        assert_eq!(mac.to_dot_notation(), address.6);

        assert_eq!(mac.to_fragments(), address.7);
        assert_eq!(mac.kind(), address.8);
        assert_eq!(mac.has_oui(), address.9);
        assert_eq!(mac.has_cid(), address.10);

        assert_eq!(mac.is_broadcast(), address.11);
        assert_eq!(mac.is_multicast(), address.12);
        assert_eq!(mac.is_unicast(), address.13);
        assert_eq!(mac.is_uaa(), address.14);
        assert_eq!(mac.is_laa(), address.15);
    }

    #[test]
    fn test_broadcast_address() {
        let address = (
            "ffffffffffff",
            "111111111111111111111111111111111111111111111111",
            281474976710655,
            "ffffffffffff",
            "ff-ff-ff-ff-ff-ff",
            "ff:ff:ff:ff:ff:ff",
            "ffff.ffff.ffff",
            ("ffffff", "ffffff"),
            "unknown",
            false,
            false,
            true,
            true,
            false,
            false,
            false,
        );

        let digits = address.0.to_string();
        let mac = MediaAccessControlAddress::new(&digits).unwrap();

        assert_eq!(mac.to_binary_representation(), address.1);
        assert_eq!(mac.to_decimal_representation(), address.2);
        assert_eq!(mac.to_plain_notation(), address.3);
        assert_eq!(mac.to_hyphen_notation(), address.4);
        assert_eq!(mac.to_colon_notation(), address.5);
        assert_eq!(mac.to_dot_notation(), address.6);

        // These tests make little sense in the context
        // of a broadcast address, but we run them for the
        // sake of completeness.
        assert_eq!(mac.to_fragments(), address.7);
        assert_eq!(mac.kind(), address.8);
        assert_eq!(mac.has_oui(), address.9);
        assert_eq!(mac.has_cid(), address.10);

        assert_eq!(mac.is_broadcast(), address.11);
        assert_eq!(mac.is_multicast(), address.12);
        assert_eq!(mac.is_unicast(), address.13);
        assert_eq!(mac.is_uaa(), address.14);
        assert_eq!(mac.is_laa(), address.15);
    }

    #[test]
    fn test_multicast_address() {
        let address = (
            "0180c2000000", // Link-Layer Discovery Protocol
            "000000011000000011000010000000000000000000000000",
            1652522221568,
            "0180c2000000",
            "01-80-c2-00-00-00",
            "01:80:c2:00:00:00",
            "0180.c200.0000",
            ("0180c2", "000000"),
            "unknown",
            false,
            false,
            false,
            true,
            false,
            false,
            false,
        );

        let digits = address.0.to_string();
        let mac = MediaAccessControlAddress::new(&digits).unwrap();

        assert_eq!(mac.to_binary_representation(), address.1);
        assert_eq!(mac.to_decimal_representation(), address.2);
        assert_eq!(mac.to_plain_notation(), address.3);
        assert_eq!(mac.to_hyphen_notation(), address.4);
        assert_eq!(mac.to_colon_notation(), address.5);
        assert_eq!(mac.to_dot_notation(), address.6);

        // These tests make little sense in the context
        // of a multicast address, but we run them for the
        // sake of completeness.
        assert_eq!(mac.to_fragments(), address.7);
        assert_eq!(mac.kind(), address.8);
        assert_eq!(mac.has_oui(), address.9);
        assert_eq!(mac.has_cid(), address.10);

        assert_eq!(mac.is_broadcast(), address.11);
        assert_eq!(mac.is_multicast(), address.12);
        assert_eq!(mac.is_unicast(), address.13);
        assert_eq!(mac.is_uaa(), address.14);
        assert_eq!(mac.is_laa(), address.15);
    }
}
