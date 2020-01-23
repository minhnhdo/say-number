# say-number
[![crates.io](https://img.shields.io/crates/v/say-number.svg)](https://crates.io/crates/say-number) [![Build Status](https://travis-ci.org/minhnhdo/say-number.svg?branch=master)](https://travis-ci.org/minhnhdo/say-number)

Library to say numbers in English, Chinese (simplified), French, Japanese and Vietnamese.

## Quickstart

Add the following to your [`Cargo.toml`](https://crates.io/) file.

```toml
[dependencies]
say-number = "1.0.0"
```

## Examples

```rust
println!("The number is {}.", say_number::say(42));
assert_eq!(say_number::say(514), "five hundred and fourteen".to_string());
assert_eq!(say_number::english::say(514), "five hundred and fourteen".to_string());
assert_eq!(say_number::chinese_simplified::say(514), "五百一十四".to_string());
assert_eq!(say_number::french::say(514), "cinq-cent-quatorze".to_string());
assert_eq!(say_number::japanese::say(514), "五百十四".to_string());
assert_eq!(say_number::vietnamese::say(514), "năm trăm mười bốn".to_string());
```
