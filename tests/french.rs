use say_number::french::say;

#[test]
fn say_0() {
    assert_eq!(say(0), "zéro".to_string());
}

#[test]
fn say_5() {
    assert_eq!(say(5), "cinq".to_string());
}

#[test]
fn say_14() {
    assert_eq!(say(14), "quatorze".to_string());
}

#[test]
fn say_21() {
    assert_eq!(say(21), "vingt-et-un".to_string());
}

#[test]
fn say_30() {
    assert_eq!(say(30), "trente".to_string());
}

#[test]
fn say_42() {
    assert_eq!(say(42), "quarante-deux".to_string());
}

#[test]
fn say_80() {
    assert_eq!(say(80), "quatre-vingts".to_string());
}

#[test]
fn say_81() {
    assert_eq!(say(81), "quatre-vingt-un".to_string());
}

#[test]
fn say_90() {
    assert_eq!(say(90), "quatre-vingt-dix".to_string());
}

#[test]
fn say_91() {
    assert_eq!(say(91), "quatre-vingt-onze".to_string());
}

#[test]
fn say_100() {
    assert_eq!(say(100), "cent".to_string());
}

#[test]
fn say_101() {
    assert_eq!(say(101), "cent-un".to_string());
}

#[test]
fn say_180() {
    assert_eq!(say(180), "cent-quatre-vingts".to_string());
}

#[test]
fn say_183() {
    assert_eq!(say(183), "cent-quatre-vingt-trois".to_string());
}

#[test]
fn say_201() {
    assert_eq!(say(201), "deux-cent-un".to_string());
}

#[test]
fn say_303() {
    assert_eq!(say(303), "trois-cent-trois".to_string());
}

#[test]
fn say_max_u8() {
    assert_eq!(
        say(std::u8::MAX as u64),
        "deux-cent-cinquante-cinq".to_string()
    );
}

#[test]
fn say_500() {
    assert_eq!(say(500), "cinq-cents".to_string());
}

#[test]
fn say_851() {
    assert_eq!(say(851), "huit-cent-cinquante-et-un".to_string());
}

#[test]
fn say_888() {
    assert_eq!(say(888), "huit-cent-quatre-vingt-huit".to_string());
}

#[test]
fn say_996() {
    assert_eq!(say(996), "neuf-cent-quatre-vingt-seize".to_string());
}

#[test]
fn say_1000() {
    assert_eq!(say(1000), "mille".to_string());
}

#[test]
fn say_1127() {
    assert_eq!(say(1127), "mille-cent-vingt-sept".to_string());
}

#[test]
fn say_2048() {
    assert_eq!(say(2048), "deux-mille-quarante-huit".to_string());
}

#[test]
fn say_3000() {
    assert_eq!(say(3000), "trois-mille".to_string());
}

#[test]
fn say_3033() {
    assert_eq!(say(3033), "trois-mille-trente-trois".to_string());
}

#[test]
fn say_3300() {
    assert_eq!(say(3300), "trois-mille-trois-cents".to_string());
}

#[test]
fn say_8888() {
    assert_eq!(
        say(8888),
        "huit-mille-huit-cent-quatre-vingt-huit".to_string()
    );
}

#[test]
fn say_max_u16() {
    assert_eq!(
        say(std::u16::MAX as u64),
        "soixante-cinq-mille-cinq-cent-trente-cinq".to_string()
    );
}

#[test]
fn say_80_000() {
    assert_eq!(say(80_000), "quatre-vingt-mille".to_string());
}

#[test]
fn say_300_000() {
    assert_eq!(say(300_000), "trois-cent-mille".to_string());
}

#[test]
fn say_311_000() {
    assert_eq!(say(311_000), "trois-cent-onze-mille".to_string());
}

#[test]
fn say_1_000_000() {
    assert_eq!(say(1_000_000), "un million".to_string());
}

#[test]
fn say_80_000_000() {
    assert_eq!(say(80_000_000), "quatre-vingts millions".to_string());
}

#[test]
fn say_100_100_400() {
    assert_eq!(
        say(100_100_400),
        "cent millions cent-mille-quatre-cents".to_string()
    );
}

#[test]
fn say_300_000_000() {
    assert_eq!(say(300_000_000), "trois-cents millions".to_string());
}

#[test]
fn say_1_000_000_000() {
    assert_eq!(say(1_000_000_000), "un milliard".to_string());
}

#[test]
fn say_1_095_371_077() {
    assert_eq!(
        say(1_095_371_077),
        "un milliard quatre-vingt-quinze millions trois-cent-soixante-et-onze-mille-soixante-dix-sept".to_string());
}

#[test]
fn say_max_u32() {
    assert_eq!(
        say(std::u32::MAX as u64),
        "quatre milliards deux-cent-quatre-vingt-quatorze millions neuf-cent-soixante-sept-mille-deux-cent-quatre-vingt-quinze".to_string()
    );
}

#[test]
fn say_1_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000), "un billion".to_string());
}

#[test]
fn say_1_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000), "un billiard".to_string());
}

#[test]
fn say_1_000_000_000_000_000_000() {
    assert_eq!(say(1_000_000_000_000_000_000), "un trillion".to_string());
}

#[test]
fn say_max_u64() {
    assert_eq!(
        say(std::u64::MAX),
        "dix-huit trillions quatre-cent-quarante-six billiards sept-cent-quarante-quatre billions soixante-treize milliards sept-cent-neuf millions cinq-cent-cinquante-et-un-mille-six-cent-quinze".to_string()
    );
}
