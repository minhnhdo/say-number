use itertools::Itertools;

static ONES: [&str; 10] = [
    "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf",
];

static TEENS: [&str; 10] = [
    "dix",
    "onze",
    "douze",
    "treize",
    "quatorze",
    "quinze",
    "seize",
    "dix-sept",
    "dix-huit",
    "dix-neuf",
];

static TENS: [&str; 8] = [
    "vingt", "trente", "quarante", "cinquante", "soixante", "soixante-dix", "quatre-vingt", "quatre-vingt-dix",
];

static BASES: [&str; 6] = [
    "mille",
    "million",
    "milliard",
    "billion",
    "billiard",
    "trillion",
];

static BASES_PLURAL: [&str; 6] = [
    "mille",
    "millions",
    "milliards",
    "billions",
    "billiards",
    "trillions",
];

static AND: &str = "et";
static DASH: &str = "-";
static HUNDRED: [&str; 2] = ["cent", "cents"];
static QUATRE_VINGTS: &str = "quatre-vingts";

/// Says a number in French.
///
/// The rules for saying numbers are from
/// [Wikipedia](https://fr.wikipedia.org/wiki/Nombres_en_français) and follow the 1990 orthography.
/// If you find any deviations from those rules, please file
/// [a bug report](https://github.com/minhnhdo/say-number/issues/new).
///
/// # Examples
///
/// ```rust
/// assert_eq!(say_number::french::say(416), "quatre-cent-seize".to_string());
/// assert_eq!(say_number::french::say(514), "cinq-cent-quatorze".to_string());
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
            if base_index == 1 && ret.len() > 0 {
                ret.push(DASH);
            }
            if hundreds_tens_and_ones == 1 {
                // single base (mille, million, milliard, ...)
                ret.push(BASES[base_index - 1]);
            } else {
                // plural base (mille, millions, milliards, ...)
                ret.push(BASES_PLURAL[base_index - 1]);
            }
        }
        let tens_and_ones = hundreds_tens_and_ones % 100;
        let hundreds = hundreds_tens_and_ones / 100;
        let ones = tens_and_ones % 10;
        let mut tens = tens_and_ones / 10;
        if hundreds_tens_and_ones > 1 && base_index == 1 {
            ret.push(DASH);
        }
        if ones > 0 {
            if tens == 1 || tens == 7 || tens == 9 {
                // special case for onze, douze,...;
                //                  soixante-et-onze, soixante-douze,..,;
                //                  quatre-vingt-onze, quatre-vingt-douze,...
                ret.push(TEENS[ones as usize]);
                tens -= 1;
            } else {
                // except for mille (not "UN mille" but just "mille")
                if base_index != 1 || hundreds_tens_and_ones != 1 {
                    ret.push(ONES[ones as usize]);
                }
            }
        }
        if tens > 0 {
            if tens != 8 && tens != 9 && ones == 1 {
                // cases for 21, 31, ..., 71 with "et-"
                ret.push(DASH);
                ret.push(AND);
            }
            if tens_and_ones == 80 && base_index != 1 {
                ret.push(QUATRE_VINGTS);
            } else {
                if ones > 0 {
                    ret.push(DASH);
                }
                ret.push(TENS[tens as usize - 2]);
            }
        }
        if hundreds > 0 {
            if tens_and_ones > 0 {
                ret.push(DASH);
            }
            if hundreds > 1 && tens_and_ones == 0 && base_index != 1 {
                // deux-centS, trois-centS, ...
                ret.push(HUNDRED[1]);
            } else {
                ret.push(HUNDRED[0]);
            }
            if hundreds > 1 {
                // number and dash before "cent", eg. DEUX-cent...
                ret.push(DASH);
                ret.push(ONES[hundreds as usize]);
            }
        }
        base_index += 1;
    }
    ret.iter().rev().map(|s| *s).intersperse(" ").collect::<String>().replace(" - ", "-")
}
