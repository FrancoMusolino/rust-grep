use std::env;
use std::fs;
use std::process;

struct Commands {
    file_path: String,
    query: String,
}

impl Commands {
    fn new(args: &[String]) -> Result<Self, String> {
        if args.len() != 2 {
            return Err(format!("Expected 2 args, found {}", args.len()));
        }

        Ok(Self {
            file_path: args[0].clone(),
            query: args[1].clone(),
        })
    }
}
fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..];

    let commands: Commands = Commands::new(args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    let contents =
        fs::read_to_string(commands.file_path).expect("Should have been able to read the file");

    println!("{contents}");
}
