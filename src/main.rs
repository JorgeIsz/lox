use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use rlox::Scanner;

fn run(source: String) -> bool {
    let mut scanner: Scanner = Scanner::new(source);
    scanner.scan_tokens();
    for token in scanner.tokens {
        println!("{}", token);
    }

    return false;
}

fn run_file(path: &str) {
    let content = fs::read_to_string(path).expect("File not found");
    let had_error = run(content);
    if had_error {
        process::exit(1);
    }
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut line).unwrap();
        run(line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
