use itertools::Itertools;

static ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

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
/// assert_eq!(say_number::english::say(416), "four hundred and sixteen".to_string());
/// assert_eq!(say_number::english::say(514), "five hundred and fourteen".to_string());
/// ```
pub fn say(mut n: u64) -> String {
    if n == 0 {
        return ONES[0].to_string();
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
        if tens_and_ones >= 10 && tens_and_ones < 20 {
            ret.push(TEENS[tens_and_ones as usize % 10]);
        } else {
            let ones = tens_and_ones % 10;
            let tens = tens_and_ones / 10;
            if ones > 0 {
                ret.push(ONES[ones as usize]);
            }
            if tens > 0 {
                ret.push(TENS[tens as usize - 2]);
            }
        }
        if hundreds > 0 {
            if tens_and_ones > 0 {
                ret.push(AND);
            }
            ret.push(HUNDRED);
            ret.push(ONES[hundreds as usize]);
        }
        if base_index == 0 && n > 0 && hundreds == 0 && tens_and_ones > 0 {
            ret.push(AND);
        }
        base_index += 1;
    }
    ret.iter().rev().intersperse(&" ").map(|s| *s).collect()
}
