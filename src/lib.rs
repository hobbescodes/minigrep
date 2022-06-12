use std::env;
use std::error::Error;
use std::fs;

// We put the two values into one struct with a meaningful name.
// This will make it easier for future maintainers of this code to understand
// how the different values relate to each other and what their purpose is
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// Logic that determines which argument foes in which variable and passes the values back to main
impl Config {
    // Returns a result with a Config instance if success, and a &'static str in the error case
    // NOTE: Our error values will always be string literals that have the 'static lifetime
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // Save the two needed arguments in variables so we can use them throughout the program
        // NOTE: The program's name takes up the first value in the vector, so we're starting at index 1
        // NOTE: The args variable in main is the owner of the argument values,
        // so we call the clone method on the values which makes a full copy of the data
        // for the Config instance to own. This does take more time and memory than storing
        // a reference to the string data, but makes the code very straightforward because we
        // don't have to manage the lifetimes of the references.
        let query = args[1].clone();
        let filename = args[2].clone();
        //using the is_err method on the Result to check whether it's an error and therefore unset
        // which means it should do a case-sensitive search, if the CASE_INSENSITIVE env variable is set to anything
        // is_err will return false and the program will perform a case-insensitive search
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Extracted function from main, to allow main to be concise and easy to verify by inspection
// Returns the unit type () in the Ok case
// Returns a type that implements the Error trait in case of an error
// NOTE: We don't have to specify what particular type the return value will be
// This gives use flexibility to return error values that may be of different types in different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the files contents
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// We need an explicit lifetime defined in the signature to tell Rust that the data returned will
// live as long as the data passed in the search function in the contents argument. The data referenced
// by a slice needs to be valid for the reference to ve valid
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
