use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Obis {
    code: [u8; 6],
    is_hex: bool,
}

impl Obis {
    pub fn convert_to_dec(&mut self) {
        self.is_hex = false;
    }

    pub fn convert_to_hex(&mut self) {
        self.is_hex = true;
    }
}

impl FromStr for Obis {
    type Err = Box<dyn Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            let dot_seperated = s.replace(":", ".").replace("-", ".").replace("*", ".");
            
            let code: [u8; 6] = dot_seperated
                .trim()
                .split('.')
                .map(|x| x.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()?
                .as_slice()
                .try_into()
                .map_err(|_| ObisError::InavlidInputDecStringLen)?;

            Ok(Self { code, is_hex: false })
        } else {
            if s.len() != 12 {
                return Err(ObisError::InavlidInputHexStringLen(s.len()).into());
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
}

impl Display for Obis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_hex {
            write!(
                f,
                "{:02X}.{:02X}.{:02X}.{:02X}.{:02X}.{:02X}",
                self.code[0], self.code[1], self.code[2], self.code[3], self.code[4], self.code[5]
            )
        } else {
            write!(
                f,
                "{}-{}:{}.{}.{}.{}",
                self.code[0], self.code[1], self.code[2], self.code[3], self.code[4], self.code[5]
            )
        }
    }
}

#[derive(Debug, PartialEq)]
enum ObisError {
    InavlidInputHexStringLen(usize),
    InavlidInputDecStringLen,
}

impl Error for ObisError {}

impl Display for ObisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObisError::InavlidInputHexStringLen(size) => {
                write!(f, "Expected 12 chars but got {}", size)
            }
            ObisError::InavlidInputDecStringLen => {
                write!(f, "Expected 6 seperated ints", )
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
        assert!(actual_error.is::<ObisError>());
        let actual_error = *actual_error.downcast::<ObisError>().unwrap();
        let expected_error = ObisError::InavlidInputHexStringLen(1);
        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_hex_obis_too_many_chars() {
        let actual_error = Obis::from_str(&"aaaaaaaaaaaaa").unwrap_err();
        assert!(actual_error.is::<ObisError>());
        let actual_error = *actual_error.downcast::<ObisError>().unwrap();
        let expected_error = ObisError::InavlidInputHexStringLen(13);
        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn test_hex_obis_bad_chars() {
        let actual_error = Obis::from_str(&"!!!!!!!!!!!!").unwrap_err();
        assert!(actual_error.is::<ParseIntError>());
    }

    #[test]
    fn test_hex_obis_display_format() {
        let obis = Obis::from_str(&"0102030405FF").unwrap();
        let obis_display = format!("{}", obis);
        assert_eq!(obis_display, "01.02.03.04.05.FF");
    }

    #[test]
    fn test_dec_obis_from_str() {
        let acutal_obis = Obis::from_str(&"1-128:7.0.14*255").unwrap();
        let expected_obis = Obis {
            code: [1, 128, 7, 0, 14, 255],
            is_hex: false,
        };
        assert_eq!(acutal_obis, expected_obis);
    }

    #[test]
    fn test_convert_obis() {
        let mut obis = Obis::from_str(&"1-128:7.0.14*255").unwrap();
        assert!(!obis.is_hex);
        obis.convert_to_hex();
        assert!(obis.is_hex);
        obis.convert_to_dec();
        assert!(!obis.is_hex);
    }
}
