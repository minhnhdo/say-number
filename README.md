# say-number
[![crates.io](https://img.shields.io/crates/v/say-number.svg)](https://crates.io/crates/say-number)

Library to say numbers in English and French.

## Quickstart

Add the following to your [`Cargo.toml`](https://crates.io/) file.

```toml
[dependencies]
say-number = "0.1"
```

## Examples

```rust
println!("The number is {}.", say_number::say(42));
assert_eq!(say_number::say(514), "five hundred and fourteen".to_string());
```
