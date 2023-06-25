mod scanner;
mod runner;

use std::{env, path::Path};
use runner::Runner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut runner = Runner::default();

    match  args.len() {
        1 => println!("REPL"), //no script file provided, run REPL
        2 => {
            runner.file(Path::new(&args[1])).unwrap_or_default();
        }, //Interpert specified script file
        _ => print!("USAGE: rlox <scriptfile>") //use usage
    };
}
