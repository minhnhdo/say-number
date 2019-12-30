use itertools::Itertools;

static ZERO_TO_TEN: [&str; 11] = [
    "không", "một", "hai", "ba", "bốn", "năm", "sáu", "bảy", "tám", "chín", "mười",
];

static BASES: [&str; 3] = ["", "ngàn", "triệu"];

static MOT: &str = "mốt";
static LAM: &str = "lăm";
static LE: &str = "lẻ";
static MUOI: &str = "mươi";
static TRAM: &str = "trăm";
static TY: &str = "tỷ";

/// Says a number in Vietnamese.
///
/// # Examples
///
/// ```rust
/// assert_eq!(say_number::vietnamese::say(416), "bốn trăm mười sáu".to_string());
/// assert_eq!(say_number::vietnamese::say(514), "năm trăm mười bốn".to_string());
/// ```
pub fn say(mut n: u64) -> String {
    if n == 0 {
        return ZERO_TO_TEN[0].to_string();
    }
    let mut ret: Vec<&str> = Vec::new();
    let mut nbillion = 0;
    while n > 0 {
        let m = n % 1_000_000_000;
        n /= 1_000_000_000;
        if m > 0 {
            (0..nbillion).for_each(|_| ret.push(TY));
        }
        say_number_less_than_a_billion(m, &mut ret);
        nbillion += 1;
    }
    ret.iter().rev().map(|s| *s).intersperse(" ").collect()
}

#[inline]
fn say_number_less_than_a_billion(mut n: u64, ret: &mut Vec<&str>) {
    let mut base_index = 0;
    while n > 0 {
        let hundreds_tens_and_ones = n % 1000;
        n /= 1000;
        if base_index > 0 && hundreds_tens_and_ones > 0 {
            ret.push(BASES[base_index]);
        }
        let tens_and_ones = hundreds_tens_and_ones % 100;
        let hundreds = hundreds_tens_and_ones / 100;
        if tens_and_ones == 0 {
            // do nothing
        } else if tens_and_ones < 10 {
            ret.push(ZERO_TO_TEN[tens_and_ones as usize]);
            if hundreds > 0 {
                ret.push(LE);
            }
        } else {
            let ones = tens_and_ones % 10;
            let tens = tens_and_ones / 10;
            if ones == 1 && tens > 1 {
                ret.push(MOT);
            } else if ones == 1 {
                /* && tens <= 1 */
                ret.push(ZERO_TO_TEN[1]);
            } else if ones == 5 {
                ret.push(LAM);
            } else if ones > 0 {
                ret.push(ZERO_TO_TEN[ones as usize]);
            }
            if tens == 1 {
                ret.push(ZERO_TO_TEN[10]);
            } else {
                ret.push(MUOI);
                ret.push(ZERO_TO_TEN[tens as usize]);
            }
        }
        if hundreds == 0 {
            if n > 0 && tens_and_ones > 0 {
                ret.push(TRAM);
                ret.push(ZERO_TO_TEN[0]);
            }
        } else {
            ret.push(TRAM);
            ret.push(ZERO_TO_TEN[hundreds as usize]);
        }
        base_index = (base_index + 1) % 3;
    }
}
