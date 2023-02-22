use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
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
                .value_name("lines")
                .long("lines")
                .short('n')
                .default_value("10")
                .help("Number of lines"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("bytes")
                .long("bytes")
                .short('c')
                .help("Number of bytes"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();

    let lines = matches
        .get_one::<String>("lines")
        .map(|v| parse_positive_int(v))
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?
        .unwrap();

    let bytes = matches
        .get_one::<String>("bytes")
        .map(|v| parse_positive_int(v))
        .transpose()
        .map_err(|e| format!("illegal bytes count -- {}", e))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(args: Config) -> MyResult<()> {
    dbg!(&args);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(v) => {
            if v > 0 {
                Ok(v)
            } else {
                Err(val.into())
            }
        }
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3usize);

    // Any string is an error
    let res = parse_positive_int("a");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "a".to_owned());

    // 0 is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_owned());
}
