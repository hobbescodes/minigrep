// Bring the std::env module into scope with a use statement so we can use its args function
// NOTE: std::env::args is nested in two levels of modules
// In cases like that it is conventional to bring the parent module into scope, rather than the function
// 1) we can easily use other functions from std::env
// 2) less ambiguous than adding std::env::args and calling the function with just args,
// because args might easily be mistaken for a function that's defined in the currnet module
// NOTE: std::env::args will panic if any argument contains invalid Unicode. If your program
// needs to accept arguments containing invalid Unicode, use std::env::args_os
// that function returns an iterator that produces OsString values instead of String values
use std::env;
// We need std::process to handle stopping and exiting the program
use std::process;
// We need to bring Config type in scope from lib.rs
mod lib;
use lib::Config;

fn main() {
    // env::args function returns an iterator, we are passing ownership of the iterator from
    // env::args to Config::new directly
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // NOTE: eprintln writes out to standard error for error messages
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // We use if let to check whether run returns an Err value and call process::exit(1) if it does
    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
