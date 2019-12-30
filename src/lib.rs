//! `say-number` says numbers in English, French and Vietnamese.
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
//!  assert_eq!(say_number::english::say(514), "five hundred and fourteen".to_string());
//! assert_eq!(say_number::french::say(514), "cinq-cent-quatorze".to_string());
//! assert_eq!(say_number::vietnamese::say(514), "năm trăm mười bốn".to_string());
//! ```

pub mod english;
pub mod french;
pub mod chinese_simplified;
pub mod vietnamese;

/// Says a number in English. **Attention**: in future releases, this function will automatically
/// select the language to use based on the user's setting on his/her machine.
///
/// # Examples
///
/// ```rust
/// assert_eq!(say_number::say(416), "four hundred and sixteen".to_string());
/// assert_eq!(say_number::say(514), "five hundred and fourteen".to_string());
/// ```
#[inline]
pub fn say(n: u64) -> String {
    english::say(n)
}
