//! `say-number` says the number in English.
//!
//! # Using this library
//!
//! Add the following to your [`Cargo.toml`](https://crates.io/):
//!
//! ```toml
//! [dependencies]
//! say-number = "0.1"
//! ```
//!
//! # Examples
//!
//! println!("The number is {}.", say_number::say(42));
//! assert_eq!(say_number::say(2048), "two thousand and forty eight".to_string());

use itertools::Itertools;
use phf::phf_map;

static ONES: phf::Map<u64, &'static str> = phf_map! {
    0u64 => "zero",
    1u64 => "one",
    2u64 => "two",
    3u64 => "three",
    4u64 => "four",
    5u64 => "five",
    6u64 => "six",
    7u64 => "seven",
    8u64 => "eight",
    9u64 => "nine",
};

static TEENS: phf::Map<u64, &'static str> = phf_map! {
    11u64 => "eleven",
    12u64 => "twelve",
    13u64 => "thirteen",
    14u64 => "fourteen",
    15u64 => "fifteen",
    16u64 => "sixteen",
    17u64 => "seventeen",
    18u64 => "eighteen",
    19u64 => "nineteen",
};

static TENS: phf::Map<u64, &'static str> = phf_map! {
    0u64 => "",
    1u64 => "ten",
    2u64 => "twenty",
    3u64 => "thirty",
    4u64 => "forty",
    5u64 => "fifty",
    6u64 => "sixty",
    7u64 => "seventy",
    8u64 => "eighty",
    9u64 => "ninety",
};

static BASES: [&str; 7] = [
    "hundred",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

static AND: &str = "and";
static HUNDRED: &str = BASES[0];

/// Says a number in English.
///
/// # Examples
///
/// ```rust
/// assert_eq!(say_number::say(416), "four hundred and sixteen".to_string());
/// assert_eq!(say_number::say(514), "five hundred and fourteen".to_string());
/// ```
pub fn say(mut n: u64) -> String {
    if n == 0 {
        return ONES[&0].to_string();
    }
    let mut ret = Vec::new();
    let mut base_index = 0;
    while n > 0 {
        let hundreds_tens_and_ones = n % 1000;
        n /= 1000;
        if hundreds_tens_and_ones > 0 && base_index != 0 {
            ret.push(BASES[base_index]);
        }
        let tens_and_ones = hundreds_tens_and_ones % 100;
        let hundreds = hundreds_tens_and_ones / 100;
        // special case for the teens
        if tens_and_ones > 10 && tens_and_ones < 20 {
            ret.push(TEENS[&tens_and_ones]);
        } else {
            let ones = tens_and_ones % 10;
            let tens = tens_and_ones / 10;
            if ones > 0 {
                ret.push(ONES[&ones]);
            }
            if tens > 0 {
                ret.push(TENS[&tens]);
            }
        }
        if hundreds > 0 {
            if tens_and_ones > 0 {
                ret.push(AND);
            }
            ret.push(HUNDRED);
            ret.push(ONES[&hundreds]);
        }
        if base_index == 0 && n > 0 && hundreds == 0 && tens_and_ones > 0 {
            ret.push(AND);
        }
        base_index += 1;
    }
    ret.iter().rev().intersperse(&" ").map(|s| *s).collect()
}

#[cfg(test)]
mod tests {
    use super::{say, ONES, TEENS, TENS};

    #[test]
    fn say_0() {
        assert_eq!(say(0), ONES[&0].to_string());
    }

    #[test]
    fn say_5() {
        assert_eq!(say(5), ONES[&5].to_string());
    }

    #[test]
    fn say_14() {
        assert_eq!(say(14), TEENS[&14].to_string());
    }

    #[test]
    fn say_30() {
        assert_eq!(say(30), TENS[&3].to_string());
    }

    #[test]
    fn say_42() {
        assert_eq!(say(42), "forty two".to_string());
    }

    #[test]
    fn say_500() {
        assert_eq!(say(500), "five hundred".to_string());
    }

    #[test]
    fn say_888() {
        assert_eq!(say(888), "eight hundred and eighty eight".to_string());
    }

    #[test]
    fn say_996() {
        assert_eq!(say(996), "nine hundred and ninety six".to_string());
    }

    #[test]
    fn say_1000() {
        assert_eq!(say(1000), "one thousand".to_string());
    }

    #[test]
    fn say_2048() {
        assert_eq!(say(2048), "two thousand and forty eight".to_string());
    }

    #[test]
    fn say_8888() {
        assert_eq!(
            say(8888),
            "eight thousand eight hundred and eighty eight".to_string()
        );
    }

    #[test]
    fn say_1_000_000() {
        assert_eq!(say(1_000_000), "one million".to_string());
    }

    #[test]
    fn say_1_000_000_000() {
        assert_eq!(say(1_000_000_000), "one billion".to_string());
    }

    #[test]
    fn say_1_000_000_000_000() {
        assert_eq!(say(1_000_000_000_000), "one trillion".to_string());
    }

    #[test]
    fn say_1_000_000_000_000_000() {
        assert_eq!(say(1_000_000_000_000_000), "one quadrillion".to_string());
    }

    #[test]
    fn say_1_000_000_000_000_000_000() {
        assert_eq!(
            say(1_000_000_000_000_000_000),
            "one quintillion".to_string()
        );
    }

    #[test]
    fn say_max_u64() {
        assert_eq!(
            say(std::u64::MAX),
            "eighteen quintillion four hundred and forty six quadrillion seven hundred and forty four trillion seventy three billion seven hundred and nine million five hundred and fifty one thousand six hundred and fifteen".to_string()
        );
    }
}
