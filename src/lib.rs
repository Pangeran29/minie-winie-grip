use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(
                "not enough argument. \n\tUse this minigrep with cargo run -- <search> <directory>",
            );
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let is_ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case: is_ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_insensitive(&config.query, &content) {
            println!("{}", line);
        }
    } else {
        for line in search_sensitive(&config.query, &content) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
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
Pick three.";

        let search_result = search_sensitive(query, contents);

        assert_eq!(vec!["safe, fast, productive."], search_result);
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";

        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Trust me.";

        let search_result = search_insensitive(query, contents);

        assert_eq!(vec!["Rust: ", "Trust me."], search_result)
    }
}
