use clap::{Arg, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    vec,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch04")
        .version("0.1.0")
        .author("huransu")
        .about("Rust head")
        .arg(
            Arg::new("bytes")
                .short('c')
                .value_name("BYTES")
                .help("Number of bytes")
                .conflicts_with("lines")
                .num_args(1),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .value_name("LINES")
                .help("Number of lines")
                .conflicts_with("bytes")
                .num_args(1)
                .default_value("10"),
        )
        .arg(
            Arg::new("files")
                .help("Files to process")
                .value_name("FILES")
                .num_args(1..)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .get_one::<String>("lines")
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?
        .unwrap();
    let bytes = matches
        .get_one::<String>("bytes")
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches
            .get_many("files")
            .unwrap()
            .map(|s: &String| s.to_string())
            .collect(),
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if file_num > 0 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    )
                }
                if let Some(size) = config.bytes {
                    read_file_bytes(&mut file, size)?;
                } else {
                    read_file(file, config.lines)?;
                }
            }
        }
    }
    Ok(())
}

fn read_file_bytes(file: &mut Box<dyn BufRead>, size: usize) -> MyResult<()> {
    let mut buffer = vec![0; size];
    file.take(size as u64).read_to_end(&mut buffer)?;
    print!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}

fn read_file(file: Box<dyn BufRead>, size: usize) -> MyResult<()> {
    for (i, line) in file.lines().enumerate() {
        if i >= size {
            break;
        }
        println!("{}", line?);
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
