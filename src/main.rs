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
    let mut res = vec![0u8; BUFFER_SIZE];
    let mut writer = BufWriter::new(stdout);
    match args.string {
        None => {
            res.write_all(b"y\n")?;
        }
        Some(string) => {
            // if the string supplied is larger than BUFFER_SIZE allocate extra space + the newline
            if res.len() < string.len() { res.reserve(BUFFER_SIZE - string.len() + 1); }
            let chars = string.as_bytes();
            res.write_all(&chars)?;
            res.write_all(&[10])?;
        }
    };
    loop {
        writer.write_all(&res)?;
    }
}