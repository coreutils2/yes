use std::io::{BufWriter, Write};
use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    pub string: Option<String>,
    #[clap(short = 'V', long)]
    pub version: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 8192;
const YES: &'static str = "y\n";

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    let mut content = vec![];
    match args.string {
        None => {
            content = Vec::from("y\n");
        }
        Some(string) => {
            content = Vec::from(string);
        }
    }
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout);
    loop {
        writer.write(&content)?;
    }
    writer.flush()?;
    anyhow::Ok(())
}
