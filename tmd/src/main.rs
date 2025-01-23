use std::{env, process};
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::Path;

static HELPSCREEN: &str = "Usage: tmd filename";
static BOLD: &str = "\x1b[1m";
static END_BOLD: &str = "\x1b[22m";
static ITALIC: &str = "\x1b[3m";
static END_ITALIC: &str = "\x1b[23m";
static NORMAL: &str = "\x1b[0m";
static QUOTE: &str = "▌";
static CODEBLOCK: &str = "\x1b[7m";
static END_CODEBLOCK: &str = "\x1b[27m";

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
    let mut code = false;
    for line in buffered.lines() {
        let mut line = line.unwrap_or(String::from(""));
        // This loop will contain the majority of the important section of our code.
        
        if line.len() == 0 {
            println!("");
            continue;
        }
        
        // First, we'll check for quotes
        while line.chars().nth(0).unwrap() == '>' {
            print!("{}", QUOTE);
            line = line[1..].to_string();
        }

        // Then, we'll render any Bold text
        while line.contains("**") {
            if bold {
                line = line.replacen("**", END_BOLD, 1);
                bold = false;
            } else {
                line = line.replacen("**", BOLD, 1);
                bold = true;
            }
        }

        // Then, we'll render any Italic text
        while line.contains("*") {
            if italic {
                line = line.replacen("*", END_ITALIC, 1);
                italic = false;
            } else {
                line = line.replacen("*", ITALIC, 1); 
                italic = true;
            }
        }

        // Detecting inline code
        while line.contains("`") {
            if code {
                line = line.replacen("`", END_CODEBLOCK, 1);
                code = false;
            } else {
                line = line.replacen("`", CODEBLOCK, 1);
                code = true;
            }
        }

        println!("{}", line);
    }

    Ok(())
}
