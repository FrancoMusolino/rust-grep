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
