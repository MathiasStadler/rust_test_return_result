// FROM HERE
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=feb31a306c38273f58ef32e7bd8937ae

use std::error::Error;
use std::process::ExitCode;

fn run() -> Result<ExitCode, Box<dyn Error>> { todo!() }

fn main() -> ExitCode {
    run().unwrap_or_else(|err| {
        eprintln!("Unexpected error: {}", err);
        ExitCode::FAILURE
    })
}

// cargo test --example  test_simple_result -- --show-output 