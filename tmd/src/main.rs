use std::{env, process};
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::Path;

static HELPSCREEN: &str = "Usage: tmd filename";
static BOLD: &str = "\x1b[1m";
static ITALIC: &str = "\x1b[3m";
static NORMAL: &str = "\x1b[0m";
static QUOTE: &str = "▌";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", HELPSCREEN);
        return Ok(()); // This will return an error in later versions
    }
    let filepath = &args[1];

    let input = File::open(filepath)?;
    let buffered = BufReader::new(input);

    let mut bold = false;
    let mut italic = false;
    for line in buffered.lines() {
        let mut line = line.unwrap_or(String::from(""));
        // This loop will contain the majority of the important section of our code.
        
        // First, we'll check for quotes
        while line.chars().nth(0).unwrap() == '>' {
            print!("{}", QUOTE);
            line = line[1..].to_string();
        }

        // Then, we'll render any Bold text
        while line.contains("**") {
            if bold {
                line = line.replacen("**", NORMAL, 1);
                bold = false;
            } else {
                line = line.replacen("**", BOLD, 1);
                bold = true;
            }
        }

        // Then, we'll render any Italic text
        while line.contains("*") {
            if italic {
                line = line.replacen("*", NORMAL, 1);
                italic = false;
            } else {
                line = line.replacen("*", ITALIC, 1); 
                italic = true;
            }
        }

        println!("{}", line);
    }

    Ok(())
}
