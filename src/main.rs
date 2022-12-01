use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use rlox::errors::LoxResult;
use rlox::interpreter::Interpreter;
use rlox::parser::Parser;

use rlox::scanner::Scanner;

fn run(source: String) -> LoxResult<()> {
    let mut scanner: Scanner = Scanner::new(source);

    scanner.scan_tokens()?;

    let tokens = scanner.tokens;
    let mut parser = Parser::new(tokens);

    let statements = parser.parse()?;
    let interpreter = Interpreter::new();

    interpreter.interpret(statements);

    Ok(())
}

fn run_file(path: &str) {
    let content = fs::read_to_string(path).expect("File not found");
    let result = run(content);
    if let Err(error) = result {
        error.report();
        process::exit(1);
    }
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut line).unwrap();
        if let Err(error) = run(line) {
            error.report();
        }
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
