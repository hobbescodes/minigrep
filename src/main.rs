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
    // collect turns the iterator into a vector containing all the values produced by the iterator.
    // we can use the collect function to create many kinds of collections, so we explicitly annotate the type of args
    let args: Vec<String> = env::args().collect();
    // We pass the whole vector to a new function associated with Config function to create an instance of Config
    // NOTE: unwrap_or_else is defined on Result<T, E> by the std library. It allows us to define some custom,
    // non-panic! error handling
    let config = Config::new(&args).unwrap_or_else(|err| {
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
