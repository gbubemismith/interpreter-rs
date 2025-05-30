mod error;
mod libs;
mod scanner;
mod token;

use error::handle_error;
use libs::Cli;

fn main() {
    if let Err(e) = Cli::start_execution() {
        handle_error(e.to_string());
    }
}
