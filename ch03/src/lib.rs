use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let mut line_number = 1;
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf_reader) => {
                if config.number_lines || config.number_nonblank_lines {
                    line_number = print_line_with_number(&config, Ok(buf_reader), line_number);
                } else {
                    print_line(Ok(buf_reader));
                }
            }
        }
    }
    Ok(())
}

pub fn print_line(buf_reader: MyResult<Box<dyn BufRead>>) {
    for line_result in buf_reader.unwrap().lines() {
        match line_result {
            Err(err) => eprintln!("Failed to read line: {}", err),
            Ok(line) => {
                println!("{}", line);
            }
        }
    }
}

pub fn print_line_with_number(
    config: &Config,
    buf_reader: MyResult<Box<dyn BufRead>>,
    mut line_number: u32,
) -> u32 {
    for line_result in buf_reader.unwrap().lines() {
        match line_result {
            Err(err) => eprintln!("Failed to read line: {}", err),
            Ok(line) => {
                if config.number_nonblank_lines && line.trim().is_empty() {
                    println!("{}", line);
                } else {
                    println!("{} {}", line_number, line);
                    line_number += 1;
                };
            }
        }
    }
    line_number
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch03")
        .version("0.1.0")
        .author("huransu")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("number")
                .value_name("number")
                .help("Number lines")
                .num_args(0)
                .short('n'),
        )
        .arg(
            Arg::new("number-nonblank")
                .value_name("number-nonblank")
                .help("Number non-blank lines")
                .num_args(0)
                .short('b'),
        )
        .get_matches();
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect();
    // .map(|word| word.);

    let number_lines = matches.get_flag("number");
    let number_nonblank_lines = matches.get_flag("number-nonblank");
    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
