#![warn(clippy::pedantic)]

use {clap::Parser, std::io::Write};

trait WriteExt: Write {
    fn write_char(&mut self, ch: char);
}

impl<T: Write> WriteExt for T {
    fn write_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let str = ch.encode_utf8(&mut buf);
        self.write_all(str.as_bytes()).unwrap();
    }
}

const FW_UC_OFFSET: u32 = 0xFF21;
const FW_LC_OFFSET: u32 = 0xFF41;
const FW_LC_ADD: u32 = FW_UC_OFFSET - 'A' as u32;
const FW_UC_ADD: u32 = FW_LC_OFFSET - 'a' as u32;

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
    for ch in input.chars() {
        let out_ch = match ch {
            'A'..='Z' => char::from_u32(ch as u32 + FW_UC_ADD).unwrap(),
            'a'..='z' => char::from_u32(ch as u32 + FW_LC_ADD).unwrap(),
            '|' => '｜',
            ' ' => char::from_u32(0x3000).unwrap(),
            _ => ch,
        };
        out.write_char(out_ch);
    }
}
