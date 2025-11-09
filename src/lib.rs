//! Word frequency library for `wordfreq-rs`.
//!
//! ```
//! use wordfreq_rs::{tokenize, count, sort_by_count_then_alpha};
//! let text = "Rust rust RUST!";
//! let tokens = tokenize(text, true);
//! let mut freqs = count(tokens);
//! let mut entries: Vec<_> = freqs.drain().collect();
//! sort_by_count_then_alpha(&mut entries);
//! assert_eq!(entries[0].0, "rust");
//! assert_eq!(entries[0].1, 3);
//! ```

#![deny(missing_docs)]

pub mod freq;
pub mod output;
pub mod sort;
pub mod tokenize;

pub use freq::count;
pub use sort::{sort_alpha, sort_by_count_then_alpha};
pub use tokenize::tokenize;
