use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, String> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get query string")),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get file nsme string")),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";

    #[test]
    fn one_result() {
        let query = "duct";

        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, CONTENTS)
        );
    }
}
