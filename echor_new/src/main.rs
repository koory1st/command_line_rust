use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("koory1st")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("omit_newline")
                .help("Do not print newline")
                .short('n')
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let omit_newline = matches.get_one::<bool>("omit_newline").unwrap();

    let ending: &str = if *omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending)
}
