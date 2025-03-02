#![warn(clippy::pedantic)]

use {clap::Parser, fw_conv::StrExt, std::io::Write};

#[derive(clap::Parser)]
struct Args {
    #[clap(long)]
    file: Option<String>,
    #[clap(long)]
    string: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = match args.file {
        Some(path) => std::fs::read_to_string(path).unwrap(),
        None => match args.string {
            Some(s) => s,
            None => panic!("Ah crap, nothing to convert"),
        },
    };
    let out = std::io::stdout();
    let mut out = out.lock();
    out.write_all(input.to_fw().as_bytes()).unwrap();
}
