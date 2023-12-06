use rust_grep::{run, Config};
use std::env;
use std::process;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = &env_args[1..];

    let config: Config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
