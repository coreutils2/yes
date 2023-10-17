use std::io::{BufWriter, Write};
use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    pub string: Option<String>,
    #[clap(short = 'V', long)]
    pub version: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 32 * 1024;
const YES: &'static str = "y\n";

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    let mut vector = vec![];
    match args.string {
        None => {
            for char in YES.chars() {
                vector.push(char as u8);
            }
        }
        Some(string) => {
            for char in string.chars() {
                vector.push(char as u8);
            }
        }
    }
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout);
    loop {
        writer.write(&vector)?;
    }
    writer.flush()?;
    anyhow::Ok(())
}
