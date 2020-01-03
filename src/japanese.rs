static ONES: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

static TEN: &str = "十";
static HUNDRED: &str = "百";
static THOUSAND: &str = "千";

static BASES: [&str; 4] = ["万", "億", "兆", "京"];

/// Says a number in Japanese.
///
/// # Examples
///
/// ```rust
/// assert_eq!(say_number::japanese::say(416), "四百十六".to_string());
/// assert_eq!(say_number::japanese::say(514), "五百十四".to_string());
/// ```
pub fn say(mut n: u64) -> String {
    if n == 0 {
        return ONES[0].to_string();
    }
    let mut ret = Vec::new();
    let mut base_index = 0;
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

        if ones > 0 {
            ret.push(ONES[ones as usize]);
        }
        if tens > 0 {
            ret.push(TEN);
            if tens > 1 {
                ret.push(ONES[tens as usize]);
            }
        }
        if hundreds > 0 {
            ret.push(HUNDRED);
            if hundreds > 1 {
                ret.push(ONES[hundreds as usize]);
            }
        }
        if thousands > 0 {
            ret.push(THOUSAND);
            if base_index > 0 || thousands > 1 {
                ret.push(ONES[thousands as usize]);
            }
        }
        base_index += 1;
    }
    ret.iter().rev().map(|s| *s).collect()
}
