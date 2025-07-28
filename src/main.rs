use std::{
    io::{IsTerminal, Read, Write},
    str::FromStr,
};

use gumdrop::Options as _;

mod hex;

/// Dehumanize text binary data into raw bytes
#[derive(Debug, gumdrop::Options)]
struct Args {
    /// Hex mode
    hex: bool,

    /// Bytes separator character
    separator: Option<char>,

    /// Show this message
    help: bool,
}

fn parse_dec<'a>(src: impl Iterator<Item = &'a str>) -> Vec<u8> {
    let out: Result<Vec<u8>, _> = src.map(FromStr::from_str).collect();
    out.unwrap()
}

fn parse_hex<'a>(src: impl Iterator<Item = &'a str>) -> Vec<u8> {
    let out: Result<Vec<u8>, _> = src
        .map(|hex| hex.strip_prefix("0x").unwrap_or(hex))
        .map(hex::decode_to_u8)
        .collect();
    out.unwrap()
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    let mut src = String::new();

    let stdin = std::io::stdin();
    if !stdin.is_terminal() {
        std::io::stdin().read_to_string(&mut src).unwrap();
    }

    let out = match args.separator {
        Some(separator) => {
            let iter = src
                .trim()
                .split(separator)
                .map(|element| element.trim())
                .filter(|element| !element.is_empty());

            if args.hex {
                parse_hex(iter)
            } else {
                parse_dec(iter)
            }
        }
        None if args.hex => {
            let out: Result<Vec<u8>, _> = src
                .trim()
                .as_bytes()
                .chunks(2)
                .map(hex::decode_to_u8)
                .collect();

            out.unwrap()
        }
        None => {
            unimplemented!()
        }
    };

    let mut stdout = std::io::stdout();
    stdout.write_all(&out).unwrap();
}
