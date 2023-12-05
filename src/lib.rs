use colored::Colorize;
use std::error::Error;
use std::fs;

pub mod config;
pub use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let matches = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if matches.len() == 0 {
        println!("No matches found");
        return Ok(());
    }

    println!("\n{}\n", "Matched lines:".bold());

    for m in matches.iter() {
        println!("{m}")
    }

    if config.flags.contains(&String::from("--verbose")) {
        println!("\n{}\n", "Verbose:".bold());

        for (i, line) in contents.lines().enumerate() {
            let i = i + 1;
            if matches.contains(&line) {
                println!("{i}. {}", line.green())
            } else {
                println!("{i}. {line}")
            }
        }
    }

    if config.flags.contains(&String::from("--enumerate")) {
        let count = matches
            .iter()
            .fold(0, |acc, line| acc + line.matches(&config.query).count());
        println!("\n{} {}\n", "Total matches:".bold(), count);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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

    #[test]
    fn should_fail_with_no_existing_file() {
        let config = Config::build(&["nowhere.txt".to_string(), "query".to_string()]).unwrap();
        let result = run(config);
        assert!(result.is_err());
    }
}
