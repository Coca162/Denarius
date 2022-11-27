use crate::error::MoneyFromStringError;
use once_cell::sync::Lazy;
use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;
use std::{fmt, ops::Sub};

#[derive(Eq, PartialEq, Debug)]
pub struct Money(pub i64);

impl Deref for Money {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Money {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(*self + *other)
    }
}

impl Add<i64> for Money {
    type Output = Self;

    fn add(self, other: i64) -> Self::Output {
        Self(*self + other)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(*self - *other)
    }
}

impl Sub<i64> for Money {
    type Output = Self;

    fn sub(self, other: i64) -> Self::Output {
        Self(*self - other)
    }
}

const PRECISION: usize = 3;
const STRING_LENGTH: usize = 2 + PRECISION;
const NEG_STRING_LENGTH: usize = STRING_LENGTH + 1;

static SINGLE_DIGIT: Lazy<String> = Lazy::new(|| {
    let mut string = String::with_capacity(STRING_LENGTH);
    string.push_str("0.00");
    string
});

static SINGLE_NEG_DIGIT: Lazy<String> = Lazy::new(|| {
    let mut string = String::with_capacity(NEG_STRING_LENGTH);
    string.push_str("-0.00");
    string
});

static DOUBLE_DIGIT: Lazy<String> = Lazy::new(|| {
    let mut string = String::with_capacity(STRING_LENGTH);
    string.push_str("0.0");
    string
});

static DOUBLE_NEG_DIGIT: Lazy<String> = Lazy::new(|| {
    let mut string = String::with_capacity(NEG_STRING_LENGTH);
    string.push_str("-0.0");
    string
});

static TRIPLE_DIGIT: Lazy<String> = Lazy::new(|| {
    let mut string = String::with_capacity(STRING_LENGTH);
    string.push_str("0.");
    string
});

static TRIPLE_NEG_DIGIT: Lazy<String> = Lazy::new(|| {
    let mut string = String::with_capacity(NEG_STRING_LENGTH);
    string.push_str("-0.");
    string
});

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string;

        let formatted = match self.0 {
            0 => "0.000",
            1..=9 => {
                string = format_from_lazy(SINGLE_DIGIT.clone(), self.0);
                &string
            }
            -9..=-1 => {
                string = format_from_lazy(SINGLE_NEG_DIGIT.clone(), self.0.abs());
                &string
            }
            10..=99 => {
                string = format_from_lazy(DOUBLE_DIGIT.clone(), self.0);
                &string
            }
            -99..=-10 => {
                string = format_from_lazy(DOUBLE_NEG_DIGIT.clone(), self.0.abs());
                &string
            }
            100..=999 => {
                string = format_from_lazy(TRIPLE_DIGIT.clone(), self.0);
                &string
            }
            -999..=-100 => {
                string = format_from_lazy(TRIPLE_NEG_DIGIT.clone(), self.0.abs());
                &string
            }
            _ => {
                string = self.0.to_string();
                let length = string.len();
                if string.ends_with("00") {
                    &string[0..length - PRECISION]
                } else {
                    string.insert(length - PRECISION, '.');
                    &string
                }
            }
        };

        f.write_str(formatted)
    }
}

fn format_from_lazy(lazy: String, value: i64) -> String {
    let mut cloned: String = lazy;
    cloned.push_str(&value.to_string());
    cloned
}

impl FromStr for Money {
    type Err = MoneyFromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !matches!(s.find('.'), Some(index) if index >= s.len() - PRECISION) {
            return Err(MoneyFromStringError::ValueTooPrecise);
        }

        let parsed = s.parse::<f64>()? * 100_f64;

        #[allow(clippy::cast_possible_truncation)]
        Ok(Money(parsed as i64))
    }
}

#[test]
fn to_str() {
    assert_eq!(Money(42000).to_string(), "42");
}

#[test]
fn to_str_decimal() {
    assert_eq!(Money(4242).to_string(), "4.242");
}

#[test]
fn to_str_zero() {
    assert_eq!(Money(0).to_string(), "0.000");
}

#[test]
fn to_str_single_digit() {
    for i in 1..=9 {
        assert_eq!(
            Money(i).to_string(),
            SINGLE_DIGIT.to_owned() + &i.to_string()
        );
    }
}

#[test]
fn to_str_single_neg_digit() {
    for i in -9..=-1 {
        assert_eq!(
            Money(i).to_string(),
            SINGLE_NEG_DIGIT.to_owned() + &i.abs().to_string()
        );
    }
}

#[test]
fn to_str_double_digit() {
    for i in 10..=99 {
        assert_eq!(
            Money(i).to_string(),
            DOUBLE_DIGIT.to_owned() + &i.to_string()
        );
    }
}

#[test]
fn to_str_double_neg_digit() {
    for i in -99..=-10 {
        assert_eq!(
            Money(i).to_string(),
            DOUBLE_NEG_DIGIT.to_owned() + &i.abs().to_string()
        );
    }
}

#[test]
fn to_str_triple_digit() {
    for i in 100..=999 {
        assert_eq!(
            Money(i).to_string(),
            TRIPLE_DIGIT.to_owned() + &i.to_string()
        );
    }
}

#[test]
fn to_str_triple_neg_digit() {
    for i in -999..=-100 {
        assert_eq!(
            Money(i).to_string(),
            TRIPLE_NEG_DIGIT.to_owned() + &i.abs().to_string()
        );
    }
}

#[test]
fn from_str() {
    assert_eq!(Money::from_str("42.50").unwrap(), Money(4250));
}

#[test]
fn from_str_err() {
    assert_eq!(Money::from_str("42.507").unwrap_err(), MoneyFromStringError::ValueTooPrecise);
}