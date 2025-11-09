//! Sorting strategies for frequencyy entries.

use std::cmp::Ordering;

/// Sort entries by count (DESC), then by word (ASC).
///
/// Works in-place on `(word, count)` tuples.
/// ```
/// use wordfreq_rs::sort::sort_by_count_then_alpha;
/// let mut v = vec![("b".to_string(),2),("a".to_string(),2),("z".to_string(),1)];
/// sort_by_count_then_alpha(&mut v);
/// assert_eq!(v, vec![("a".into(),2),("b".into(),2),("z".into(),1)]);
/// ```
pub fn sort_by_count_then_alpha(entries: &mut Vec<(String, u64)>) {
    entries.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        other => other,
    });
}

/// Sort entries by word (ascending alphanumerically)
pub fn sort_alpha(entries: &mut Vec<(String, u64)>) {
    entries.sort_by(|a, b| a.0.cmp(&b.0));
}
