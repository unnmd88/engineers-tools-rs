use async_snmp::Value;
use std::net::Ipv4Addr;
use validator::{Validate, ValidationError};

type Parser = fn(Value) -> Option<String>;

#[derive(Debug)]
pub struct OidDefinition {
    name: String,
    oid: String,
    parser: Parser,
}

struct Errors {
    field: String,
    message: String,
}

enum ControllerTypes {
    Swarco,
    Peek,
    PotokS,
    PotokP,
    Signal,
}

fn validate_ipv4(value: &str) -> Result<(), ValidationError> {
    match value.parse::<Ipv4Addr>() {
        Ok(_) => Ok(()),
        Err(_) => {
            let mut err = ValidationError::new("invalid_ipv4");
            err.message = Some("Некорректный ipv4 адрес".into());
            Err(err)
        }
    }
}

// Client -> Server
#[derive(Validate)]
pub struct RawPollingConfig {
    pub name: String,

    #[validate(custom(function = "validate_ipv4",))]
    pub ip: String,
    pub port: u16,
    pub monitor_id: u8,
    pub community: String,
    pub interval: u16,
    pub controller_type: String,
}

pub struct PollingConfig {
    pub name: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub monitor_id: u8,
    pub community: String,
    pub interval: u16,
    pub controller_type: ControllerTypes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ipv4_return_ok() {
        let ip_addresses = ["192.168.0.1", "0.0.0.0", "10.179.54.9"];

        for ip in ip_addresses {
            assert!(validate_ipv4(ip).is_ok(), "Failed for: {}", ip);
        }
    }

    #[test]
    fn test_validate_ipv4_return_err() {
        let bad_ip = ["192.1.1.", "abraa", "1.1111.123.45", "193.13.133.14.1"];

        for ip in bad_ip {
            let err = validate_ipv4(ip).unwrap_err();

            assert_eq!(err.code, "invalid_ipv4");
            assert_eq!(err.message, Some("Некорректный ipv4 адрес".into()));
        }
    }
}
