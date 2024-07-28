use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub text: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let text = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            text,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.text, &contents)
    } else {
        search(&config.text, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(text: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(text) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(text: &str, contents: &'a str) -> Vec<&'a str> {
    let text = text.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&text) {
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
        let text = "ember";
        let contents = "\
Gym memberships
are getting
way too expensive.
Ember.";

        assert_eq!(vec!["Gym memberships"], search(text, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "TOo";
        let contents = "\
Gym memberships
Are getting
Way too expensive.";

        assert_eq!(
            vec!["Way too expensive."],
            search_case_insensitive(query, contents)
        );
    }
}
