use std::{
    fs,
    io::{stdin, stdout, Write},
};

use clap::Parser;

use crate::error::Error;

#[derive(Debug, Parser)]
#[command(name = "interpreter-rs", version = "0.1.0", author = "Gbubemi Smith")]
pub struct Cli {
    /// The path to the file to read
    path: Option<std::path::PathBuf>,
}

impl Cli {
    pub fn start_execution() -> Result<(), Error> {
        let args = Self::parse();

        match args.path {
            Some(path) => Self::run_file(&path)?,
            None => Self::run_prompt(),
        }

        Ok(())
    }

    pub fn run_file(path: &std::path::PathBuf) -> Result<(), Error> {
        let file_ext = path.extension();

        match file_ext {
            Some(ext) => {
                if ext != "lox" {
                    return Err(Error::Runtime(
                        "Required '.lox' file, file not supported!".to_string(),
                    ));
                }
            }
            None => return Err(Error::Runtime("Invalid file extension".to_string())),
        }

        let content = fs::read_to_string(path)?;
        Self::run(&content);
        Ok(())
    }

    // run interactively
    pub fn run_prompt() {
        loop {
            print!("> ");
            if let Err(e) = stdout().flush() {
                eprintln!("Error: {}", e);
                break;
            }

            let mut line = String::new();
            match stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    Self::run(&line);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }
    }

    fn run(content: &str) {}
}
