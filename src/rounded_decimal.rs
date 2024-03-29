use std::error;
use std::fmt;
use std::fmt::Display;
use std::ops;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct RoundedDecimal {
    value: i64,
    places: usize,
}

type RoundedDecimalParseResult = std::result::Result<RoundedDecimal, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        RoundedDecimal { value, places: 0 }
    }
}

impl FromStr for RoundedDecimal {
    type Err = Box<dyn error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = value.split('.').collect();
        let len = v.len();

        if len == 0 {
            panic!("Never come.");
        } else if len == 1 {
            let value: i64 = value.parse()?;

            RoundedDecimalParseResult::Ok(RoundedDecimal { value, places: 0 })
        } else if len == 2 {
            let places: usize = v[1].len();
            let left: i64 = v[0].parse()?;
            let right: i64 = v[1].parse()?;
            let value = left * 10i64.pow(places as u32) + right;

            RoundedDecimalParseResult::Ok(RoundedDecimal { value, places })
        } else {
            RoundedDecimalParseResult::Err(Box::new(RoundedDecimalParseError {
                description: "Invalid dot count.",
            }))
        }
    }
}

impl Display for RoundedDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn zero_aligned(value: i64, places: usize) -> String {
            let value_str = value.to_string();
            let mut zeros = "".to_owned();
            for _ in value_str.len()..places {
                zeros += "0"
            }
            zeros + &value_str
        }

        if self.places == 0 {
            write!(f, "{}", self.value)
        } else {
            let left = self.value / 10i64.pow(self.places as u32);
            let right = self.value - left;
            write!(f, "{}.{}", left, zero_aligned(right, self.places))
        }
    }
}

impl ops::Add<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn add(self, rhs: RoundedDecimal) -> RoundedDecimal {
        if self.places != rhs.places {
            panic!("Places mismatch. : {} + {}", self, rhs);
        }

        RoundedDecimal {
            value: self.value + rhs.value,
            places: self.places,
        }
    }
}

impl ops::AddAssign<RoundedDecimal> for RoundedDecimal {
    fn add_assign(&mut self, rhs: RoundedDecimal) {
        *self = *self + rhs;
    }
}

impl ops::Sub<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn sub(self, rhs: RoundedDecimal) -> RoundedDecimal {
        if self.places != rhs.places {
            panic!("Places mismatch. : {} - {}", self, rhs);
        }

        RoundedDecimal {
            value: self.value - rhs.value,
            places: self.places,
        }
    }
}

impl ops::SubAssign<RoundedDecimal> for RoundedDecimal {
    fn sub_assign(&mut self, rhs: RoundedDecimal) {
        *self = *self - rhs;
    }
}

impl ops::Mul<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn mul(self, rhs: RoundedDecimal) -> RoundedDecimal {
        let scale = 10i64.pow(rhs.places as u32);
        RoundedDecimal {
            value: self.value * rhs.value / scale,
            places: self.places,
        }
    }
}

impl ops::MulAssign<RoundedDecimal> for RoundedDecimal {
    fn mul_assign(&mut self, rhs: RoundedDecimal) {
        *self = *self * rhs;
    }
}

impl ops::Div<RoundedDecimal> for RoundedDecimal {
    type Output = RoundedDecimal;

    fn div(self, rhs: RoundedDecimal) -> RoundedDecimal {
        let scale = 10i64.pow(rhs.places as u32);
        RoundedDecimal {
            value: self.value * scale / rhs.value,
            places: self.places,
        }
    }
}

impl ops::DivAssign<RoundedDecimal> for RoundedDecimal {
    fn div_assign(&mut self, rhs: RoundedDecimal) {
        *self = *self / rhs;
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
        fn from_str_0_01() {
            let n = from("0.01");

            assert_eq!(
                RoundedDecimal {
                    value: 1,
                    places: 2
                },
                n
            );

            assert_eq!("0.01", format!("{}", n))
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
        fn add_assign() {
            let mut value = from("0.1");
            value += from("0.1");
            assert_eq!(from("0.2"), value);
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
        fn sub_assign() {
            let mut value = from("0.2");
            value -= from("0.1");
            assert_eq!(from("0.1"), value);
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
        fn mul_assign() {
            let mut value = from("0.1");
            value *= from("2");
            assert_eq!(from("0.2"), value);
        }

        #[test]
        fn div() {
            assert_eq!(from("0.0"), from("0.1") / from("2"));
            assert_eq!(from("20"), from("2") / from("0.1"));
        }

        #[test]
        fn div_assign() {
            let mut value = from("2");
            value /= from("0.1");
            assert_eq!(from("20"), value);
        }
    }
}
