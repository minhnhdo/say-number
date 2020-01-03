static ONES: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

static TWO: &str = "两";
static TEN: &str = "十";
static HUNDRED: &str = "百";
static THOUSAND: &str = "千";

static BASES: [&str; 4] = ["万", "亿", "兆", "京"];

/// Says a number in Chinese (Simplified).
///
/// # Examples
///
/// ```rust
/// assert_eq!(say_number::chinese_simplified::say(416), "四百一十六".to_string());
/// assert_eq!(say_number::chinese_simplified::say(514), "五百一十四".to_string());
/// ```
pub fn say(mut n: u64) -> String {
    if n == 0 {
        return ONES[0].to_string();
    }
    let mut ret = Vec::new();
    let mut base_index = 0;
    let mut ling = true; // whether the previous group of digits end with "ling"
    while n > 0 {
        let thousands_hundreds_tens_and_ones = n % 10000;
        n /= 10000;
        if thousands_hundreds_tens_and_ones > 0 && base_index != 0 {
            ret.push(BASES[base_index - 1]);
        }
        let hundreds_tens_and_ones = thousands_hundreds_tens_and_ones % 1000;
        let thousands = thousands_hundreds_tens_and_ones / 1000;

        let tens_and_ones = hundreds_tens_and_ones % 100;
        let hundreds = hundreds_tens_and_ones / 100;

        let ones = tens_and_ones % 10;
        let tens = tens_and_ones / 10;

        if thousands_hundreds_tens_and_ones > 0 {
            if ones > 0 {
                if thousands_hundreds_tens_and_ones == 2 {
                    ret.push(TWO);
                } else {
                    ret.push(ONES[ones as usize]);
                }
            }
            if tens > 0 {
                ret.push(TEN);
                if !(tens == 1
                    && thousands_hundreds_tens_and_ones < 20
                    && base_index == 0
                    && n == 0)
                {
                    ret.push(ONES[tens as usize]);
                }
            } else if (thousands_hundreds_tens_and_ones > 99 || n > 0) && ones > 0 {
                // number in the form of ab0c, where a != 0 || b != 0
                // or ...000a
                ret.push(ONES[0]);
            }
            if hundreds > 0 {
                ret.push(HUNDRED);
                if hundreds == 2 {
                    ret.push(TWO);
                } else {
                    ret.push(ONES[hundreds as usize]);
                }
            } else if tens != 0 && (thousands > 0 || n > 0) && tens_and_ones > 0 {
                // have not said "ling" from the ten digit
                // number in the form of a0bc, where a != 0
                // or ...00ab
                ret.push(ONES[0]);
            }
            if thousands > 0 {
                ret.push(THOUSAND);
                if thousands == 2 {
                    ret.push(TWO);
                } else {
                    ret.push(ONES[thousands as usize]);
                }
            } else if n > 0 && hundreds_tens_and_ones > 0 && hundreds != 0 {
                // have not said "ling" from the hundred digit
                // number in the form of ...0abc
                ret.push(ONES[0]);
            }
        } else if ling == false {
            ret.push(ONES[0]);
        }
        ling = thousands_hundreds_tens_and_ones < 1000;
        base_index += 1;
    }
    ret.iter().rev().map(|s| *s).collect()
}
