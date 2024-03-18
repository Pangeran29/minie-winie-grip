use std::{
    env::{self, Args},
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().expect("Query expected");
        let file_path = args.next().expect("File path expected");
        let is_ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case: is_ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    println!("File: {}", &config.file_path);

    if config.ignore_case {
        for line in search_insensitive(&config.query, &content) {
            println!("{}", line.trim());
        }
        println!();
    } else {
        for line in search_sensitive(&config.query, &content) {
            println!("{}", line.trim());
        }
        println!();
    }

    Ok(())
}

pub fn visit_dir(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = visit_dir(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
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
    content.lines().filter(|item| item.to_lowercase().contains(query)).collect()
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

    #[test]
    fn read_from_dir() {
        let config = Config {
            file_path: "./src".to_string(),
            query: "print".to_string(),
            ignore_case: true,
        };

        let result = visit_dir(config.file_path);
        let result = match result {
            Ok(result) => result,
            Err(_) => todo!(),
        };

        let path_buf1 = Path::new("./src/main.rs").to_path_buf();
        let path_buf2 = Path::new("./src/lib.rs").to_path_buf();

        assert_eq!(vec![path_buf1, path_buf2], result)
    }
}
