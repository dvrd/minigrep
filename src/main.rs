use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let config = Config::new(&mut env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("application error: {err}");
        std::process::exit(1);
    }
}
