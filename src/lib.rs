use colored::Colorize;
use std::error::Error;
use std::fs;
pub struct Config {
    file_path: String,
    query: String,
    flags: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Expected at least 2 args");
        }

        Ok(Self {
            file_path: args[0].clone(),
            query: args[1].clone(),
            flags: args[2..].to_vec(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let matches = search(&config.query, &contents);

    for m in matches.iter() {
        println!("{m}")
    }

    if config.flags.contains(&String::from("--verbose")) {
        println!();
        println!("{}", "Verbose:".bold());
        println!();

        for line in contents.lines() {
            if matches.contains(&line) {
                println!("{}", line.green())
            } else {
                println!("{line}")
            }
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
