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
    let mut buffer_size = 0;
    let mut stdout = std::io::stdout();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    let mut content;
    match args.string {
        None => {
            content = String::from("y\n");
            buffer_size = content.len() * 2;
        }
        Some(string) => {
            content = string;
            buffer_size = content.len() * 2;
        }
    }
    let mut writer = BufWriter::with_capacity(buffer_size, stdout);
    loop {
        writer.write(content.as_bytes())?;
    }
    writer.flush()?;
    anyhow::Ok(())
}
