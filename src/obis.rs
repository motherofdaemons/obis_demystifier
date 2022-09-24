use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Obis {
    code: [u8; 6],
    is_hex: bool,
}

impl Obis {}

impl FromStr for Obis {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 12 {
            return Err(HexObisError::InavlidInputStringLen(s.len()).into());
        }
        let a = u8::from_str_radix(&s[..2], 16)?;
        let b = u8::from_str_radix(&s[2..4], 16)?;
        let c = u8::from_str_radix(&s[4..6], 16)?;
        let d = u8::from_str_radix(&s[6..8], 16)?;
        let e = u8::from_str_radix(&s[8..10], 16)?;
        let f = u8::from_str_radix(&s[10..12], 16)?;
        Ok(Self {
            code: [a, b, c, d, e, f],
            is_hex: true,
        })
    }
}

impl Display for Obis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}.{:02X}.{:02X}.{:02X}.{:02X}.{:02X}",
            self.code[0], self.code[1], self.code[2], self.code[3], self.code[4], self.code[5]
        )
    }
}

#[derive(Debug, PartialEq)]
enum HexObisError {
    InavlidInputStringLen(usize),
}

impl Error for HexObisError {}

impl Display for HexObisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HexObisError::InavlidInputStringLen(size) => {
                write!(f, "Expected 12 chars but got {}", size)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn test_hex_obis_from_str() {
        let actual_obis = Obis::from_str(&"ffffffffffff").unwrap();
        let expected_obis = Obis {
            code: [255, 255, 255, 255, 255, 255],
            is_hex: true,
        };
        assert_eq!(actual_obis, expected_obis);
    }

    #[test]
    fn test_hex_obis_too_few_chars() {
        let actual_error = Obis::from_str(&"a").unwrap_err();
        assert!(actual_error.is::<HexObisError>());
        let actual_error = *actual_error.downcast::<HexObisError>().unwrap();
        let expected_error = HexObisError::InavlidInputStringLen(1);
        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_hex_obis_too_many_chars() {
        let actual_error = Obis::from_str(&"aaaaaaaaaaaaa").unwrap_err();
        assert!(actual_error.is::<HexObisError>());
        let actual_error = *actual_error.downcast::<HexObisError>().unwrap();
        let expected_error = HexObisError::InavlidInputStringLen(13);
        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_hex_obis_bad_chars() {
        let actual_error = Obis::from_str(&"!!!!!!!!!!!!").unwrap_err();
        assert!(actual_error.is::<ParseIntError>());
    }
}