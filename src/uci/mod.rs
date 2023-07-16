use log;
use lexer::Lexer;

pub mod lexer;

pub fn main() {
    let mut lex: Lexer = Lexer::default();

    loop {
        let mut input: String = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                lex.interpret(input);
            }
            Err(error) => {
                log::error!("Error reading input: {}", error);
                std::process::exit(1);
            }
        }
    }
}
