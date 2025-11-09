//! Rendering utilities (table / JSON)

use serde_json::json;

/// Render entries as a left-aligned text table
pub fn render_table(entries: &[(String, u64)]) -> String {
    if entries.is_empty() {
        return "No entries found.".to_string();
    }

    let max_word_len: usize = entries
        .iter()
        .map(|(word, _)| word.len())
        .max()
        .unwrap_or(4);
    let mut out = String::new();

    out.push_str(&format!("{:<width$} COUNT\n", "WORD", width = max_word_len));
    out.push_str(&format!("{:-<w$} {:-<5}\n", "", "", w = max_word_len));
    for (word, count) in entries {
        out.push_str(&format!(
            "{:<width$} {}\n",
            word,
            count,
            width = max_word_len
        ));
    }

    out
}

/// Render entries as pretty JSON array:
/// [{"word": "foo", "count": 3}, ...]
pub fn render_json(entries: &[(String, u64)]) -> serde_json::Result<String> {
    let v: Vec<_> = entries
        .iter()
        .map(|(word, count)| json!({ "word": word, "count": count }))
        .collect();
    serde_json::to_string_pretty(&v)
}
