use std::io::{BufWriter, Write};
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
    let size;
    let mut writer = BufWriter::new(stdout);
    let content = match args.string {
        None => {
            format!("y\n")
            // write!(buffer, "y\n")?;
            // buffer.flush()?;
        }
        Some(string) => {
            format!("{string}\n")
            // write!(buffer, "{string}\n")?;
            // buffer.flush()?;
        }
    };
    if content.len() < BUFFER_SIZE {
        size = BUFFER_SIZE;
    } else {
        size = content.len().next_power_of_two();
    }
    let mut buffer = vec![0u8; size];
    buffer.flush()?;
    write!(buffer, "{content}")?;
    loop {
        writer.write_all(&buffer)?;
        writer.flush()?;
    }
}
