use say_number::say;

#[test]
fn say_0() {
    assert_eq!(say(0), "zero".to_string());
}

#[test]
fn say_5() {
    assert_eq!(say(5), "five".to_string());
}

#[test]
fn say_14() {
    assert_eq!(say(14), "fourteen".to_string());
}

#[test]
fn say_30() {
    assert_eq!(say(30), "thirty".to_string());
}

#[test]
fn say_42() {
    assert_eq!(say(42), "forty two".to_string());
}

#[test]
fn say_max_u8() {
    assert_eq!(
        say(std::u8::MAX as u64),
        "two hundred and fifty five".to_string()
    );
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
fn say_max_u16() {
    assert_eq!(
        say(std::u16::MAX as u64),
        "sixty five thousand five hundred and thirty five".to_string()
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
fn say_max_u32() {
    assert_eq!(
        say(std::u32::MAX as u64),
        "four billion two hundred and ninety four million nine hundred and sixty seven thousand two hundred and ninety five".to_string()
    );
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
