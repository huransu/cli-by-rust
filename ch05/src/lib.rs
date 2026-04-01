use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    pub num_lines: usize,
    pub num_words: usize,
    pub num_bytes: usize,
    pub num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch05")
        .version("0.1.0")
        .author("huransu")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .help("Prints help information")
                .short('l')
                .long("lines")
                .num_args(0),
        )
        .arg(
            Arg::new("words")
                .help("Show word count")
                .short('w')
                .long("words")
                .num_args(0),
        )
        .arg(
            Arg::new("bytes")
                .help("Show byte count")
                .short('c')
                .long("bytes")
                .num_args(0)
                .conflicts_with("chars"),
        )
        .arg(
            Arg::new("chars")
                .help("Show character count")
                .short('m')
                .num_args(0)
                .conflicts_with("bytes"),
        )
        .get_matches();
    let files = matches
        .get_many("files")
        .unwrap()
        .map(|s: &String| s.to_string())
        .collect::<Vec<String>>();
    let lines = matches.get_flag("lines");
    let words = matches.get_flag("words");
    let bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");
    let is_any_flag_set = lines || words || bytes || chars;
    Ok(Config {
        files,
        lines: !is_any_flag_set ^ lines,
        words: !is_any_flag_set ^ words,
        bytes: !is_any_flag_set ^ bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    for filename in &config.files {
        let result = count(open(filename)?)?;
        print_info(&config, &result, filename);
        total.num_lines += result.num_lines;
        total.num_words += result.num_words;
        total.num_bytes += result.num_bytes;
        total.num_chars += result.num_chars;
    }
    if config.files.len() > 1 {
        print_info(&config, &total, "total");
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    while file.read_line(&mut line)? > 0 {
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line.len();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn print_info(config: &Config, info: &FileInfo, filename: &str) {
    let count = if config.bytes {
        info.num_bytes
    } else {
        info.num_chars
    };
    let filename = if filename == "-" { "" } else { filename };
    println!(
        "{:>8}{:>8}{:>8} {}",
        info.num_lines, info.num_words, count, filename
    );
}
