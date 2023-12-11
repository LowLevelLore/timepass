mod errors;
mod expressions;
mod interpreter;
mod parser;
mod scanner;
mod statements;
mod tests;

use interpreter::Interpreter;
use parser::Parser;

use crate::scanner::*;
use colored::Colorize;
use std::io::{self, Write};

use std::{ffi::OsStr, path::Path};

fn open_file(filename: String) -> Result<String, String> {
    match std::fs::read_to_string(filename.clone()) {
        Ok(content) => return Ok(content),
        Err(_e) => Err("File not found !".to_string()),
    }
}

fn run_file(filename: String) -> Result<(), String> {
    match open_file(filename) {
        Ok(contents) => {
            println!("{}", contents);
            match run(contents) {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn run(contents: String) -> Result<(), String> {
    let mut sc = Scanner::new(contents.as_bytes());
    match sc.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.clone());
            let mut interpreter = Interpreter::new();
            match parser.parse() {
                Ok(statements) => {
                    match interpreter.interpret(statements) {
                        Ok(_) => (),
                        Err(e) => return Err(e.to_string()),
                    }

                    return Ok(());
                }
                Err(e) => {
                    return Err(format!("{}", e));
                }
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

fn check_extension(filename: String, extension: String) -> bool {
    let file_ext = (Path::new(filename.as_str())
        .extension()
        .and_then(OsStr::to_str))
    .map(str::to_string)
    .unwrap();
    return file_ext == extension;
}

fn open_shell() -> Result<(), String> {
    #[allow(while_true)]
    while true {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Error while flushing stdout !".to_string()),
        }
        let mut buffer: String = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                buffer = buffer.trim().to_string();
                if buffer == "exit()" || buffer == "quit()" {
                    break;
                }
            }
            Err(_) => return Err("Unable to read from buffer!".to_string()),
        }
        match run(buffer.clone()) {
            Ok(_) => {
                buffer.clear();
                println!("");
            }
            Err(msg) => {
                eprintln!("{} : {}", "ERROR".red(), msg);
            }
        }
    }
    return Ok(());
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let args: Vec<_> = std::env::args().collect();
    let lang_name: String = "timepass".to_string();
    let extension: String = "tp".to_string();
    let filename: String;
    if args.len() == 1 {
        match open_shell() {
            Ok(_) => {
                // do nothing
            }
            Err(msg) => {
                eprintln!("ERROR : {}", msg);
            }
        }
        // println!(
        //     "Expected a filename, \nTo get help, type : {} -help",
        //     lang_name
        // );
    }
    if args.len() > 1 {
        filename = args[1].clone();
        if filename == "-help" {
            println!(
                "Correct usage : {} relative/path/to/file.{}",
                lang_name, extension
            );
        } else {
            if check_extension(filename.clone(), extension.clone()) {
                match run_file(filename) {
                    Ok(_) => {
                        println!("FINISED RUNNING FILE");
                    }
                    Err(msg) => {
                        eprintln!("ERROR : {}", msg);
                    }
                }
            } else {
                eprintln!("ERROR : File must be in the format of .{}", extension);
            }
        }
    }
}
