use anyhow::{Context, Result};
use clap::{ArgAction, Parser, ValueEnum};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wordfreq-rs", about = "Count word frequencies from text")]
struct Args {
    /// Input file (default: read from sdin)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Case-insensitive counting (lowercase all words)
    #[arg(long, default_value_t = true, action = ArgAction::SetTrue)]
    ignore_case: bool,

    /// Minimum word length to include
    #[arg(long, default_value_t = 1)]
    min_len: usize,

    /// Print only the top N words (0 = all)
    #[arg(long, default_value_t = 0)]
    top: usize,

    /// Short output by: count or alpha
    #[arg(long, value_enum, default_value_t = SortBy::Count)]
    sort_by: SortBy,

    /// Output as JSON instead of text table
    #[arg(long)]
    json: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SortBy {
    Count,
    Alpha,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read input
    let input = match &args.file {
        Some(p) => {
            fs::read_to_string(p).with_context(|| format!("Failed to read file: {:?}", p))?
        }
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("stdin read failed")?;
            buffer
        }
    };

    // Tokenize -> count
    let mut counts: HashMap<String, u64> = HashMap::new();
    for word in tokenize(&input, args.ignore_case) {
        if word.len() >= args.min_len {
            // If the word hasn't encountered before, insert with count 0, then increment
            *counts.entry(word).or_insert(0) += 1;
        }
    }

    // Move to vector for sorting
    let mut entries: Vec<(String, u64)> = counts.into_iter().collect();
    match args.sort_by {
        SortBy::Count => {
            // Descending by count, then ascending by word for stable tie-break
            entries.sort_by(|a, b| match b.1.cmp(&a.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            });
        }
        SortBy::Alpha => {
            // Ascending by word (alphabetical)
            entries.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }

    // Apply top N if requested
    if args.top > 0 && args.top < entries.len() {
        entries.truncate(args.top);
    }

    // Output
    if args.json {
        // Compact JSON array like: [{"word": "foo", "count": 3}, ...]
        let json_vec: Vec<_> = entries
            .iter()
            .map(|(word, count)| serde_json::json!({ "word": word, "count": count }))
            .collect();
        println!("{}", serde_json::to_string(&json_vec)?);
    } else {
        // Show the result as simple table instead
        if entries.is_empty() {
            println!("(no words)");
            return Ok(());
        }

        // Determine width for alignment
        let max_word_len = entries
            .iter()
            .map(|(word, _)| word.len())
            .max()
            .unwrap_or(4);
        println!("{:<width$} COUNT", "WORD", width = max_word_len);
        println!("{:-<w$} {:-<5}", "", "", w = max_word_len);
        for (word, count) in entries {
            println!("{:<width$} {}", word, count, width = max_word_len);
        }
    }

    Ok(())
}

/// Tokenize input into words.
/// Splits on any non-alphanumeric characters (Unicode-aware).
fn tokenize(input: &str, ignore_case: bool) -> impl Iterator<Item = String> + '_ {
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
