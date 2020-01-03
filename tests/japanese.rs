use say_number::japanese::say;

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
    assert_eq!(say(std::u8::MAX as u64), "二百五十五".to_string());
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
    assert_eq!(say(1000), "千".to_string());
}

#[test]
fn say_1001() {
    assert_eq!(say(1001), "千一".to_string());
}

#[test]
fn say_2048() {
    assert_eq!(say(2048), "二千四十八".to_string());
}

#[test]
fn say_8888() {
    assert_eq!(say(8888), "八千八百八十八".to_string());
}

#[test]
fn say_10_101() {
    assert_eq!(say(10_101), "一万百一".to_string());
}

#[test]
fn say_max_u16() {
    assert_eq!(say(std::u16::MAX as u64), "六万五千五百三十五".to_string());
}

#[test]
fn say_1_000_000() {
    assert_eq!(say(1_000_000), "百万".to_string());
}

#[test]
fn say_1_000_010() {
    assert_eq!(say(1_000_010), "百万十".to_string());
}

#[test]
fn say_1_001_000() {
    assert_eq!(say(1_001_000), "百万千".to_string());
}

#[test]
fn say_1_010_000() {
    assert_eq!(say(1_010_000), "百一万".to_string());
}

#[test]
fn say_1_100_010() {
    assert_eq!(say(1_100_010), "百十万十".to_string());
}

#[test]
fn say_10_000_000() {
    assert_eq!(say(10_000_000), "一千万".to_string());
}

#[test]
fn say_10_050_026() {
    assert_eq!(say(10_050_026), "一千五万二十六".to_string());
}

#[test]
fn say_1_000_000_000() {
    assert_eq!(say(1_000_000_000), "十億".to_string());
}

#[test]
fn say_1_000_000_010() {
    assert_eq!(say(1_000_000_010), "十億十".to_string());
}

#[test]
fn say_max_u32() {
    assert_eq!(
        say(std::u32::MAX as u64),
        "四十二億九千四百九十六万七千二百九十五".to_string()
    );
}

#[test]
fn say_1_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000), "一兆".to_string());
}

#[test]
fn say_1_000_010_000_000() {
    assert_eq!(say(1_000_010_000_000), "一兆一千万".to_string());
}

#[test]
fn say_20_200_200_200_022() {
    assert_eq!(
        say(20_200_200_200_022),
        "二十兆二千二億二十万二十二".to_string()
    );
}

#[test]
fn say_1_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000), "一千兆".to_string());
}

#[test]
fn say_1_000_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000_000), "百京".to_string());
}

#[test]
fn say_max_u64() {
    assert_eq!(
        say(std::u64::MAX),
        "一千八百四十四京六千七百四十四兆七百三十七億九百五十五万千六百十五".to_string()
    );
}
