//! Frequency counting

use std::collections::HashMap;

/// Count occurrences of tokens into a `HashMap<String, u64>`.
///
/// ```
/// use wordfreq_rs::{tokenize, count};
/// let tokens = tokenize("a a b", true);
/// let m = count(tokens);
/// assert_eq!(m.get("a"), Some(&2));
/// assert_eq!(m.get("b"), Some(&1));
/// ```
pub fn count<I>(tokens: I) -> HashMap<String, u64>
where
    I: IntoIterator<Item = String>,
    // I is a generic type parameter. It can be any type,
    // as long as it implements IntoIterator with Item = String.
{
    let mut counts: HashMap<String, u64> = HashMap::new();
    for token in tokens {
        *counts.entry(token).or_insert(0) += 1;
    }
    counts
}
