use anyhow::{Context, Result};
use clap::{ArgAction, Parser, ValueEnum};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};
use wordfreq_rs::output::{render_json, render_table};
use wordfreq_rs::{count, sort_alpha, sort_by_count_then_alpha, tokenize};

#[derive(Parser, Debug)]
#[command(name = "wordfreq-rs", about = "Count word frequencies from text")]
struct Args {
    /// Input file (default: read from stdin)
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

    /// Sort output by: count or alpha
    #[arg(long, value_enum, default_value_t = SortBy::Count)]
    sort_by: SortBy,

    /// Output as JSON instead of text table
    #[arg(long)]
    json: bool,

    /// Output as CSV instead of text table
    #[arg(long)]
    csv: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SortBy {
    Count,
    Alpha,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Input
    let input = match &args.file {
        Some(p) => {
            fs::read_to_string(p).with_context(|| format!("failed to read: {}", p.display()))?
        }
        None => {
            let mut s = String::new();
            io::stdin()
                .read_to_string(&mut s)
                .context("stdin read failed")?;
            s
        }
    };

    // Pipeline
    let tokens = tokenize(&input, args.ignore_case);
    let mut freqs = count(tokens);
    freqs.retain(|word, _| word.len() >= args.min_len); // drop short words

    let mut entries: Vec<(String, u64)> = freqs.drain().collect();
    match args.sort_by {
        SortBy::Count => sort_by_count_then_alpha(&mut entries),
        SortBy::Alpha => sort_alpha(&mut entries),
    }
    if args.top > 0 && args.top < entries.len() {
        entries.truncate(args.top);
    }

    // Output
    if args.json {
        println!("{}", render_json(&entries)?);
    } else if args.csv {
        println!("{}", wordfreq_rs::output::render_csv(&entries)?);
    } else {
        print!("{}", render_table(&entries));
    }
    Ok(())
}
