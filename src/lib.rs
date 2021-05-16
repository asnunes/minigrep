use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let lines = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    let mut result: Vec<&'a str> = Vec::new();
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

    mod case_sensitive {
        use super::*;

        #[test]
        fn no_result() {
            let query = "this is not in the text bellow";
            let contents = "\
this is text is so exclusive
no one is going to find any string in here...
";

            let result: Vec<&str> = vec![];
            assert_eq!(result, search(query, contents))
        }

        #[test]
        fn one_result() {
            let query = "rough";
            let contents = "\
meow, meow, meow! The cat said wisely.
rough, rough, rough! The dog said happily.
";

            assert_eq!(
                vec!["rough, rough, rough! The dog said happily."],
                search(query, contents)
            )
        }

        #[test]
        fn two_results() {
            let query = "but";
            let contents = "\
I should write a line here, but what should I say?
I should also write another one because it is two results, but.... This line will match?
And this? This should not match!
";

            assert_eq!(
            vec![
                "I should write a line here, but what should I say?",
                "I should also write another one because it is two results, but.... This line will match?"
                ],
            search(query, contents))
        }
    }
    mod case_insensitive {
        use super::*;

        #[test]
        fn no_result() {
            let query = "this is not in the text bellow";
            let contents = "\
this is text is so exclusive
no one is going to find any string in here...
";

            let result: Vec<&str> = vec![];
            assert_eq!(result, search_case_insensitive(query, contents))
        }

        #[test]
        fn one_result() {
            let query = "RuST";
            let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust me.
";

            assert_eq!(
                vec!["Rust:", "Trust me."],
                search_case_insensitive(query, contents)
            )
        }
    }
}
