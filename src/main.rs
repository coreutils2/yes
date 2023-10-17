use std::io::{Write};
use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    pub string: Option<Vec<u8>>,
    #[clap(short = 'V', long)]
    pub version: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 8192;

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    // let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout);
    match args.string {
        None => {
            const YES: &str = "y\n";
            loop {
                stdout.write(YES.as_ref())?;
            }
        }
        Some(vector) => {
            loop {
                stdout.write(&vector)?;
            }
        }
    }
    // anyhow::Ok(())
}
