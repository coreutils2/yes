use std::io::{Write};
use clap::{Command, arg};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 4096;

fn main() -> anyhow::Result<()> {
    let app = Command::new(NAME)
        .disable_version_flag(true)
        .disable_help_flag(true)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(arg!(-V --version "output version information and exit").action(clap::ArgAction::Version))
        .arg(arg!(-h --help "display this help and exit").action(clap::ArgAction::HelpLong))
        .arg(arg!([STRING] "string to print, (if none is passed print 'y')"))
        .get_matches();
    let mut vector = vec![0u8; BUFFER_SIZE];
    match app.get_one::<String>("STRING") {
        None => {
            vector.write_all(&[79, 10][..])?;
        }
        Some(string) => {
            if string.len() > BUFFER_SIZE { vector.reserve(BUFFER_SIZE - string.len() + 1); }
            let chars = string.as_bytes();
            vector.write_all(&chars)?;
            vector.write_all(&[10])?; // newline char
        }
    };
    let mut stdout = std::io::stdout().lock();
    // let mut writer = BufWriter::new(stdout);
    loop {
        stdout.write_all(&vector)?;
        stdout.flush()?;
    }
}