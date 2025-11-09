//! Tokenization utilities.

/// Split input into “words”.
///
/// - Splits on any non-alphanumeric char (Unicode aware).
/// - Lowercases if `ignore_case` is true.
///
/// ```
/// use wordfreq_rs::tokenize;
/// let v: Vec<_> = tokenize("Hi, Rust!", true).collect();
/// assert_eq!(v, vec!["hi","rust"]);
/// ```
pub fn tokenize(input: &str, ignore_case: bool) -> impl Iterator<Item = String> + '_ {
    input
        .split(|ch: char| !ch.is_alphabetic())
        .filter(|s| !s.is_empty())
        .map(move |s| {
            if ignore_case {
                s.to_lowercase()
            } else {
                s.to_string()
            }
        })
}
