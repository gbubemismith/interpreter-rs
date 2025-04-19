use core::fmt;
use std::{io, process};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parser(clap::Error),
    Runtime(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Error::Parser(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::Parser(err) => write!(f, "{}", err),
            Error::Runtime(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn handle_error(error: String) {
    eprintln!("{}", error);
    process::exit(1);
}

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, r#where: &str, message: &str) {
    let err = format!("[Line {}] Error {}: {}", line, r#where, message);
    eprintln!("{}", err);
}
