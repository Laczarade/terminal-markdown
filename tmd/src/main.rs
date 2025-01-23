use std::{env, process};
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let input = File::open(filepath)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
