use crate::error::ValidationError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteSize(pub u64);

impl FromStr for ByteSize {
    type Err = ValidationError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(ValidationError::InvalidByteSize {
                input: input.into(),
                reason: "Empty size".into(),
            });
        }

        let input = input.trim();
        let (num, suffix) = input.split_at(input.len() - 1);

        let value: u64 = num.parse().map_err(|_| ValidationError::InvalidByteSize {
            input: input.into(),
            reason: "Invalid number".into(),
        })?;

        if value == 0 {
            return Err(ValidationError::InvalidByteSize {
                input: input.into(),
                reason: "Size must be greater than zero".into(),
            });
        }

        let multiplier = match suffix.to_ascii_uppercase().as_str() {
            "B" => 1,
            "K" => 1024,
            "M" => 1024_u64.pow(2),
            "G" => 1024_u64.pow(3),
            "T" => 1024_u64.pow(4),
            _ => {
                return Err(ValidationError::InvalidByteSize {
                    input: input.into(),
                    reason: "Unknown suffix".into(),
                })
            }
        };

        Ok(ByteSize(value * multiplier))
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0;

        let (value, suffix) = if bytes >= 1024_u64.pow(4) {
            (bytes / 1024_u64.pow(4), "T")
        } else if bytes >= 1024_u64.pow(3) {
            (bytes / 1024_u64.pow(3), "G")
        } else if bytes >= 1024_u64.pow(2) {
            (bytes / 1024_u64.pow(2), "M")
        } else if bytes >= 1024 {
            (bytes / 1024, "K")
        } else {
            (bytes, "B")
        };

        write!(f, "{}{}", value, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_sizes() {
        assert_eq!(ByteSize::from_str("100G").unwrap().0, 100 * 1024_u64.pow(3));
        assert_eq!(ByteSize::from_str("512M").unwrap().0, 512 * 1024_u64.pow(2));
        assert_eq!(ByteSize::from_str("1T").unwrap().0, 1024_u64.pow(4));
        assert_eq!(ByteSize::from_str("1024K").unwrap().0, 1024 * 1024);
        assert_eq!(ByteSize::from_str("2048B").unwrap().0, 2048);
    }

    #[test]
    fn parse_lowercase_sizes() {
        assert_eq!(ByteSize::from_str("100g").unwrap().0, 100 * 1024_u64.pow(3));
        assert_eq!(ByteSize::from_str("512m").unwrap().0, 512 * 1024_u64.pow(2));
    }

    #[test]
    fn reject_invalid_sizes() {
        assert!(ByteSize::from_str("").is_err());
        assert!(ByteSize::from_str("abc").is_err());
        assert!(ByteSize::from_str("-100G").is_err());
        assert!(ByteSize::from_str("100X").is_err());
        assert!(ByteSize::from_str("G100").is_err());
        assert!(ByteSize::from_str("0G").is_err());
    }

    #[test]
    fn display_formatting() {
        let size = ByteSize(1024_u64.pow(3));
        assert_eq!(size.to_string(), "1G");
    }
}
