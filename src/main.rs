use std::io::{BufWriter, StdoutLock, Write};
use clap::{CommandFactory, Parser};


#[derive(Debug, Parser)]
#[clap(disable_help_flag = true)]
struct CliArgs {
    /// string to print (if none is passed print 'y')
    pub string: Option<String>,
    /// output version information and exit
    #[clap(short = 'V', long)]
    pub version: bool,
    /// display this help and exit
    #[clap(short, long)]
    help: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_SIZE: usize = 4096;

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout().lock();
    let args = CliArgs::parse();
    // this if statement only exists so I can have a custom help message
    if args.help {
        let mut cli = CliArgs::command();
        cli.build();
        cli.print_help()?;
        std::process::exit(0);
    }
    if args.version {
        writeln!(stdout, "{VERSION_STRING}")?;
        return anyhow::Ok(());
    }
    let mut res: Vec<u8>;
    let mut writer = BufWriter::new(stdout);
    match args.string {
        None => {
            res = vec![0u8; BUFFER_SIZE];
            res.write_all(b"y\n")?;
        }
        Some(string) => {
            if string.len() <= BUFFER_SIZE { res = vec![0u8; BUFFER_SIZE]; } else { res = vec![0u8; string.len().next_power_of_two()]; }
            write!(res, "{string}\n")?;
        }
    };
    write_to_stdout(writer, &res)?;
    anyhow::Ok(())
}

#[inline]
fn write_to_stdout(mut writer: BufWriter<StdoutLock>, output: &Vec<u8>) -> anyhow::Result<()> {
    while writer.write_all(&output).is_ok() {
        writer.flush()?;
    }
    anyhow::Ok(())
}
