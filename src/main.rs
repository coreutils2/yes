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
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout);
    match args.string {
        None => {
            loop {
                writer.write(b"y\n")?;
            }
        }
        Some(string) => {
            let temp = format!("{string}\n");
            loop {
                writer.write(temp.as_bytes())?;
            }
        }
    };
    writer.flush()?;
    anyhow::Ok(())
}
