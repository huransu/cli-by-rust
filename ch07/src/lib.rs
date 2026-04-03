use crate::EntryType::*;
use clap::{Arg, ArgAction, Command, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Debug, Eq, PartialEq, ValueEnum)]
enum EntryType {
    #[value(name = "f")]
    File,
    #[value(name = "d")]
    Directory,
    #[value(name = "l")]
    Symlink,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch07")
        .version("0.1.0")
        .author("huransu")
        .about("Rust find")
        .arg(
            Arg::new("name")
                .help("Name")
                .action(ArgAction::Append)
                .short('n')
                .long("name")
                .num_args(0..),
        )
        .arg(
            Arg::new("type")
                .help("Entry type")
                .action(ArgAction::Append)
                .short('t')
                .long("type")
                .value_parser(clap::value_parser!(EntryType))
                .num_args(0..),
        )
        .arg(
            Arg::new("path")
                .help("Search paths")
                .num_args(0..)
                .default_value("."),
        )
        .get_matches();
    let names = matches
        .get_many::<String>("name")
        .unwrap_or_default()
        .map(|vals| Regex::new(vals).map_err(|_| format!("Invalid --name \"{}\"", vals)))
        .collect::<Result<Vec<_>, _>>()?;
    let paths = matches
        .get_many::<String>("path")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();
    let types = matches
        .get_many::<EntryType>("type")
        .unwrap_or_default()
        .map(|t| t.to_owned())
        .collect();
    Ok(Config {
        paths,
        names,
        entry_types: types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if config.names.is_empty() {
                        print_by_entry_type(&config.entry_types, &entry);
                        continue;
                    }
                    for regex in &config.names {
                        if regex.is_match(entry.file_name().to_str().unwrap_or_default()) {
                            print_by_entry_type(&config.entry_types, &entry);
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_by_entry_type(types: &[EntryType], entry: &walkdir::DirEntry) {
    if types.is_empty() {
        println!("{}", entry.path().display())
    }
    for entry_type in types {
        match entry_type {
            File if entry.file_type().is_file() => {
                println!("{}", entry.path().display())
            }
            Directory if entry.file_type().is_dir() => {
                println!("{}", entry.path().display())
            }
            Symlink if entry.file_type().is_symlink() => {
                println!("{}", entry.path().display())
            }
            _ => (),
        }
    }
}
