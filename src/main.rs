use linereader::LineReader;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str;

fn process_file(file: impl Read) -> Result<(), Box<dyn Error>> {
    let mut reader = LineReader::new(file);
    let mut index = 0;
    while let Some(line) = reader.next_line() {
        let line = str::from_utf8(line?)?;
        let line = line.replace(",", "|");
        match index == 0 {
            true => {
                let count = line.matches('|').count();
                let mut bar = String::new();
                for _ in 0..=count {
                    bar.push_str("--|");
                }
                println!("{}", line.trim());
                println!("{}", bar.trim());
            }
            false => {
                println!("{}", line.trim());
            }
        }
        index += 1;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 || &args[1] == "-" {
        process_file(io::stdin())?
    } else {
        for _ in 1..args.len() {
            let file = File::open(&args[1])?;
            process_file(file)?;
        }
    }

    Ok(())
}
