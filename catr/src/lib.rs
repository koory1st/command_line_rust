use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_noblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Koory1st")
        .about("Rust Cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input File(s)")
                .default_value("-")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("number")
                .help("Number lines")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .help("Number non-blank lines")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let vec = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();
    let number = matches.get_one::<bool>("number").unwrap().clone();
    let number_noblank = matches.get_one::<bool>("number_nonblank").unwrap().clone();
    Ok(Config {
        files: vec,
        number_lines: number,
        number_noblank_lines: number_noblank,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for file_name in config.files {
        let f_result = open(&file_name);

        if let Err(e) = open(&file_name) {
            eprintln!("{}: {}", file_name, e);
            return Ok(());
        }

        let f = f_result.unwrap();
        let reader = BufReader::new(f);

        let mut line_no = 0;
        for line in reader.lines() {
            let line = line?;

            if !config.number_noblank_lines || line.len() > 0 {
                line_no += 1;
            }

            if (config.number_lines || config.number_noblank_lines) && line.len() > 0 {
                println!("{:>6}\t{}", line_no, line);
            } else {
                println!("{}", line);
            }
        }
    }
    Ok(())
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}
