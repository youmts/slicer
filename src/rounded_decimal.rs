use std::error;
use std::fmt;
use std::fmt::Display;
use std::ops;
use std::str::FromStr;

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
            value,
            places: 0,
        }
    }
}

impl FromStr for RoundedDecimal {
    type Err = Box<dyn error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = value.split('.').collect();
        let len = v.len();

        if len == 0 {
            panic!("Never come");
        } else if len == 1 {
            let value: i64 = value.parse()?;

            RoundedDecimalParseResult::Ok(
                RoundedDecimal {
                    value,
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
                    value,
                    places,
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

impl ops::Add<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn add(self, _rhs: RoundedDecimal) -> RoundedDecimal {
        if self.places != _rhs.places {
            panic!("places mismatch");
        }

        RoundedDecimal {value: self.value + _rhs.value, places: self.places}
    }
}

impl ops::Sub<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn sub(self, _rhs: RoundedDecimal) -> RoundedDecimal {
        if self.places != _rhs.places {
            panic!("places mismatch");
        }

        RoundedDecimal {value: self.value - _rhs.value, places: self.places}
    }
}

impl ops::Mul<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn mul(self, _rhs: RoundedDecimal) -> RoundedDecimal {
        let n = 10i64.pow(_rhs.places as u32);
        RoundedDecimal {value: self.value * _rhs.value / n, places: self.places}
    }
}

impl ops::Div<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn div(self, _rhs: RoundedDecimal) -> RoundedDecimal {
        let n = 10i64.pow(_rhs.places as u32);
        RoundedDecimal {value: self.value * n / _rhs.value, places: self.places}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(value: &str) -> RoundedDecimal {
        RoundedDecimal::from_str(value).ok().unwrap()
    }

    mod constructors {
        use super::*;

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
            let n = from("0.1");

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
            let n = from("0.10");

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
            let n = from("1");

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

    mod operators {
        use super::*;

        #[test]
        fn add() {
            assert_eq!(from("0.2"), from("0.1") + from("0.1"));
        }

        #[test]
        #[should_panic]
        fn add_different_places() {
            let _ = from("0.1") + from("0.01");
        }

        #[test]
        fn sub() {
            assert_eq!(from("0.1"), from("0.2") - from("0.1"));
        }

        #[test]
        #[should_panic]
        fn sub_different_places() {
            let _ = from("0.2") - from("0.01");
        }

        #[test]
        fn mul() {
            assert_eq!(from("0.2"), from("0.1") * from("2"));
            assert_eq!(from("0"), from("2") * from("0.1"));
        }

        #[test]
        fn div() {
            assert_eq!(from("0.0"), from("0.1") / from("2"));
            assert_eq!(from("20"), from("2") / from("0.1"));
        }
    }
}

