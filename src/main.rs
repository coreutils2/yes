use std::io::{Write};
use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    pub string: Option<String>,
    #[clap(short = 'V', long)]
    pub version: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 8192;

const YES: &str = "y";

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    // let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout);
    let output;
    match args.string {
        None => {
            loop {
                writeln!(stdout, "{}", YES)?;
            }
        }
        Some(string) => {
            output = format!("{string}\n");
        }
    }
    anyhow::Ok(())
}
