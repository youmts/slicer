use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct RoundedDecimal {
    value: i64,
    places: usize,
}

type RoundedDecimalParseResult = std::result::Result<RoundedDecimal, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub struct RoundedDecimalParseError {
    description: &'static str,
}

impl Display for RoundedDecimalParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.description)
    }
}

impl error::Error for RoundedDecimalParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl RoundedDecimal {
    pub fn from(value: i64) -> Self {
        RoundedDecimal {
            value: value,
            places: 0,
        }
    }

    pub fn from_str(value: &str) -> RoundedDecimalParseResult {
        let v: Vec<&str> = value.split('.').collect();
        let len = v.len();

        if len == 0 {
            panic!("Never come");
        } else if len == 1 {
            let value: i64 = value.parse()?;

            RoundedDecimalParseResult::Ok(
                RoundedDecimal {
                    value: value,
                    places: 0,
                },
            )
        } else if len == 2 {
            let places: usize = v[1].len();
            let left: i64 = v[0].parse()?;
            let right: i64 = v[1].parse()?;
            let value = left * 10i64.pow(places.try_into()?) + right;

            RoundedDecimalParseResult::Ok(
                RoundedDecimal {
                    value: value,
                    places: places,
                },
            )
        } else {
            RoundedDecimalParseResult::Err(
                Box::new(
                    RoundedDecimalParseError {description: "Invalid dot number."}
                )
            )
        }
    }
}

impl Display for RoundedDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.places == 0 {
            write!(f, "{}", self.value)
        } else {
            let left = self.value / 10i64.pow(self.places.try_into().unwrap());
            let right = self.value - left;
            write!(f, "{}.{}", left, right)
        }
    }
}

#[cfg(test)]
mod tests {
    mod constructors {
        use super::super::*;

        #[test]
        fn from_1() {
            let n = RoundedDecimal::from(1);
            assert_eq!(
                RoundedDecimal {
                    value: 1,
                    places: 0
                },
                n
            );
            assert_eq!("1", format!("{}", n))
        }

        #[test]
        fn from_str_0_1() {
            let n = RoundedDecimal::from_str("0.1").ok().unwrap();

            assert_eq!(
                RoundedDecimal {
                    value: 1,
                    places: 1
                },
                n
            );

            assert_eq!("0.1", format!("{}", n))
        }

        #[test]
        fn from_str_0_10() {
            let n = RoundedDecimal::from_str("0.10").ok().unwrap();

            assert_eq!(
                RoundedDecimal {
                    value: 10,
                    places: 2
                },
                n
            );

            assert_eq!("0.10", format!("{}", n))
        }

        #[test]
        fn from_str_1() {
            let n = RoundedDecimal::from_str("1").ok().unwrap();

            assert_eq!(
                RoundedDecimal {
                    value: 1,
                    places: 0
                },
                n
            );

            assert_eq!("1", format!("{}", n))
        }

        #[test]
        fn from_str_not_number() {
            RoundedDecimal::from_str("a").err().unwrap();
            RoundedDecimal::from_str("a.a").err().unwrap();
        }
    }
}

