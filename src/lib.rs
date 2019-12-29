//! `say-number` says numbers in English.
//!
//! # Quickstart
//!
//! Add the following to your [`Cargo.toml`](https://crates.io/) file.
//!
//! ```toml
//! [dependencies]
//! say-number = "0.1"
//! ```
//!
//! # Examples
//!
//! ```rust
//! println!("The number is {}.", say_number::say(42));
//! assert_eq!(say_number::say(514), "five hundred and fourteen".to_string());
//! ```

pub mod english;
pub mod french;

#[inline]
pub fn say(n: u64) -> String {
    english::say(n)
}
