use std::env;

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
    pub flags: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Expected at least 2 args");
        }

        Ok(Self {
            file_path: args[0].clone(),
            query: args[1].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
            flags: args[2..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fail_with_less_than_2_args() {
        let config = Config::build(&[String::from("dummy.txt")]);
        assert!(config.is_err())
    }

    #[test]
    fn should_build_with_2_or_more_args() {
        let config = Config::build(&[String::from("dummy.txt"), String::from("nobody")]);
        assert!(config.is_ok());
    }

    #[test]
    fn should_properly_assign_values() {
        let config = Config::build(&[String::from("dummy.txt"), String::from("nobody")]).unwrap();
        assert_eq!("dummy.txt", config.file_path);
        assert_eq!("nobody", config.query);
    }
}
