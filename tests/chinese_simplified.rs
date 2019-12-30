use say_number::chinese_simplified::say;

#[test]
fn say_0() {
    assert_eq!(say(0), "零".to_string());
}

#[test]
fn say_5() {
    assert_eq!(say(5), "五".to_string());
}

#[test]
fn say_14() {
    assert_eq!(say(14), "十四".to_string());
}

#[test]
fn say_30() {
    assert_eq!(say(30), "三十".to_string());
}

#[test]
fn say_42() {
    assert_eq!(say(42), "四十二".to_string());
}

#[test]
fn say_max_u8() {
    assert_eq!(
        say(std::u8::MAX as u64),
        "两百五十五".to_string()
    );
}

#[test]
fn say_500() {
    assert_eq!(say(500), "五百".to_string());
}

#[test]
fn say_888() {
    assert_eq!(say(888), "八百八十八".to_string());
}

#[test]
fn say_996() {
    assert_eq!(say(996), "九百九十六".to_string());
}

#[test]
fn say_1000() {
    assert_eq!(say(1000), "一千".to_string());
}

#[test]
fn say_2048() {
    assert_eq!(say(2048), "两千零四十八".to_string());
}

#[test]
fn say_8888() { assert_eq!(say(8888), "八千八百八十八".to_string()
    );
}

#[test]
fn say_10101() { assert_eq!(say(10101), "一万零一百零一".to_string()
);
}

#[test]
fn say_max_u16() {
    assert_eq!(
        say(std::u16::MAX as u64),
        "六万五千五百三十五".to_string()
    );
}

#[test]
fn say_1_000_000() {
    assert_eq!(say(1_000_000), "一百万".to_string());
}

#[test]
fn say_10_050_026() {
    assert_eq!(say(10_050_026), "一千零五万零二十六".to_string());
}

#[test]
fn say_1_000_000_000() {
    assert_eq!(say(1_000_000_000), "一十亿".to_string());
}

#[test]
fn say_max_u32() {
    assert_eq!(
        say(std::u32::MAX as u64),
        "四十二亿九千四百九十六万七千两百九十五".to_string()
    );
}

#[test]
fn say_1_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000), "一兆".to_string());
}

#[test]
fn say_1_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000), "一千兆".to_string());
}

#[test]
fn say_1_000_000_000_000_000_000() {
    assert_eq!(
        say(1_000_000_000_000_000_000),
        "一百京".to_string()
    );
}

#[test]
fn say_max_u64() {
    assert_eq!(
        say(std::u64::MAX),
        "一千八百四十四京六千七百四十四兆零七百三十七亿零九百五十五万一千六百一十五".to_string()
    );
}
