use std::io::{BufWriter, Write};
use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    pub string: Option<String>,
    #[clap(short = 'V', long)]
    pub version: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 4 * 1024 * 1024;
const YES: &'static str = "y\n";

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout().lock();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    let content = match args.string {
        None => {
            String::from("y\n")
        }
        Some(string) => { String::from(string) }
    };
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout);
    let mut vector: Vec<u8> = vec![0; content.len()];
    for c in content.chars() {
        vector.push(c as u8);
    }
    loop {
        writer.write(&vector)?;
    }
    writer.flush()?;
    anyhow::Ok(())
}
