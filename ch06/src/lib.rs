use clap::{Arg, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch06")
        .version("0.1.0")
        .author("huransu")
        .about("Rust uniq")
        .arg(
            Arg::new("count")
                .help("Show counts")
                .short('c')
                .long("count")
                .num_args(0),
        )
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .num_args(1)
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .num_args(1),
        )
        .get_matches();
    let count = matches.get_flag("count");
    let in_file = matches
        .get_one::<String>("in_file")
        .ok_or("Missing input file")?
        .to_string();
    let out_file = matches.get_one::<String>("out_file").map(|s| s.to_string());
    Ok(Config {
        in_file,
        out_file,
        count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", &config.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    let mut line = String::new();
    let mut is_first_line = true;
    let mut prev_line = String::new();
    let mut line_count = 1;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if is_first_line {
            is_first_line = false;
            prev_line = line.clone();
        } else if line != prev_line {
            if config.count {
                write!(out_file, "{:>4} {}", line_count, prev_line)?;
            } else {
                write!(out_file, "{}", prev_line)?;
            }
            line_count = 1;
        } else {
            line_count += 1;
        }
        prev_line = line.clone();
        line.clear();
    }
    if line_count > 0 {
        if config.count {
            write!(out_file, "{:>4} {}", line_count, prev_line)?;
        } else {
            write!(out_file, "{}", prev_line)?;
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
