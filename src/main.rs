use std::io::{self, prelude::*};
use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let f = fs::File::open(&args.path)?;
    let mut reader = io::BufReader::new(f);

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
