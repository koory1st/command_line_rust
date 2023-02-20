use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("1.0.0")
        .author("Levy")
        .about("Rust Head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .help("print the first K bytes of each file"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .help("print the first K lines instead of the first 10;"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("FILES")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();

    Ok(Config {
        files: files,
        lines: (),
        bytes: (),
    })
}
