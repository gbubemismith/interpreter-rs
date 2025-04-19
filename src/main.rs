mod error;
mod libs;
mod scanner;
mod token;

use error::handle_error;
use libs::Args;

fn main() {
    if let Err(e) = Args::start_execution() {
        handle_error(e.to_string());
    }
}
