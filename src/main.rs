use std::io::{ self, BufRead };

use regex::Regex;
use clap::{ Parser };

use crate::file_read::read::open_file;
mod file_read;
fn main() {
    let args = Args::parse();
    if let Some(v) = args.path {
        let haystack = open_file(&v).join("\n");

        grep_with_regex(&haystack, &args.needle);
        return;
    }

    println!("Enter the haystack");
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut haystack = String::new();

    let _ = reader.read_line(&mut haystack);
    grep_with_regex(&haystack, &args.needle);
}

fn grep_with_regex(haystack: &str, needle: &str) {
    let reg = Regex::new(needle).unwrap();

    for line in haystack.lines() {
        let contains_string = reg.find(line);
        match contains_string {
            Some(value) => {
                println!("start: {}\nend: {}", value.start(), value.end());
            }
            None => {
                continue;
            }
        }
        let words: Vec<_> = line
            .split(" ")
            .map(|word| {
                if word == needle { format!("[{word}]") } else { word.to_string() }
            })
            .collect();
        println!("haystack: {}\n", words.join(" "));
    }
}

/// Grep-lite is a cli based search tool that helps users to search for a pattern within a given text
#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about,
    long_about = "Grep-lite is a cli based search tool that helps users to search for a pattern within a given text"
)]
struct Args {
    /// File to search
    #[arg(short = 'p', long = "path")]
    path: Option<String>,
    /// Needle defines the word you want to search for
    #[arg(short = 'n', long = "needle")]
    needle: String,
}

/* 
grep_with_regex(
        "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what picture ?
It is the same with books.
What picture do we seek through millions of pages?",
        "picture"
    )
*/
