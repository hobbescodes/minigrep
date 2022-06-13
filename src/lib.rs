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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // The first value in the return value of env::args is the name of the program which we want to ignore
        args.next();
        // Call next to get the value we want to put in the query field and use match to extract the value
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        // Call next to get the value we want to put in the filename field and use match to extract the value
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
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
    // Get the lines of the contents, and use the filter adaptor to keep only the lines where line.contains(query) returns true
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
