use say_number::vietnamese::say;

#[test]
fn say_0() {
    assert_eq!(say(0), "không".to_string());
}

#[test]
fn say_5() {
    assert_eq!(say(5), "năm".to_string());
}

#[test]
fn say_11() {
    assert_eq!(say(11), "mười một".to_string());
}

#[test]
fn say_15() {
    assert_eq!(say(15), "mười lăm".to_string());
}

#[test]
fn say_31() {
    assert_eq!(say(31), "ba mươi mốt".to_string());
}

#[test]
fn say_42() {
    assert_eq!(say(42), "bốn mươi hai".to_string());
}

#[test]
fn say_max_u8() {
    assert_eq!(
        say(std::u8::MAX as u64),
        "hai trăm năm mươi lăm".to_string()
    );
}

#[test]
fn say_500() {
    assert_eq!(say(500), "năm trăm".to_string());
}

#[test]
fn say_888() {
    assert_eq!(say(888), "tám trăm tám mươi tám".to_string());
}

#[test]
fn say_996() {
    assert_eq!(say(996), "chín trăm chín mươi sáu".to_string());
}

#[test]
fn say_1000() {
    assert_eq!(say(1000), "một ngàn".to_string());
}

#[test]
fn say_2048() {
    assert_eq!(say(2048), "hai ngàn không trăm bốn mươi tám".to_string());
}

#[test]
fn say_8888() {
    assert_eq!(say(8888), "tám ngàn tám trăm tám mươi tám".to_string());
}

#[test]
fn say_max_u16() {
    assert_eq!(
        say(std::u16::MAX as u64),
        "sáu mươi lăm ngàn năm trăm ba mươi lăm".to_string()
    );
}

#[test]
fn say_1_000_000() {
    assert_eq!(say(1_000_000), "một triệu".to_string());
}

#[test]
fn say_1_000_000_000() {
    assert_eq!(say(1_000_000_000), "một tỷ".to_string());
}

#[test]
fn say_max_u32() {
    assert_eq!(
        say(std::u32::MAX as u64),
        "bốn tỷ hai trăm chín mươi bốn triệu chín trăm sáu mươi bảy ngàn hai trăm chín mươi lăm"
            .to_string()
    );
}

#[test]
fn say_1_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000), "một ngàn tỷ".to_string());
}

#[test]
fn say_1_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000), "một triệu tỷ".to_string());
}

#[test]
fn say_1_000_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000_000), "một tỷ tỷ".to_string());
}

#[test]
fn say_max_u64() {
    assert_eq!(
        say(std::u64::MAX),
        "mười tám tỷ tỷ bốn trăm bốn mươi sáu triệu bảy trăm bốn mươi bốn ngàn không trăm bảy mươi ba tỷ bảy trăm lẻ chín triệu năm trăm năm mươi mốt ngàn sáu trăm mười lăm".to_string()
    );
}
