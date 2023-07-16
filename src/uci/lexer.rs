use std::str::FromStr;
use log;
use chess::{ChessMove, Board};
use crate::strategies::search::Search;

pub struct Lexer {
    search_strategy: Option<Search>,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            search_strategy: None,
        }
    }
}

impl Lexer {
    pub fn interpret(&mut self, input: String) {
        if input.starts_with("position") {
            return self.position(input);
        }

        if input.starts_with("go") {
            return self.calculate_best_move();
        }
    
        match input.trim() {
            "uci" => self.init(),
            "isready" => self.is_ready(),
            "quit" => self.quit(),
            "ucinewgame" => self.new_game(),
            _ => log::warn!("Unknown command: {}", input.trim()),
        }
    }

    fn new_game(&mut self) {
        // TODO: Implement
    }

    fn position(&mut self, input: String) {
        // Example position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        if !input.contains("fen") {
            return;
        }

        let fen_text: Option<&str> = input.split("fen").collect::<Vec<&str>>().pop();
        if fen_text.is_none() {
            return;
        }

        let board = Board::from_str(fen_text.unwrap().trim()).unwrap();
        self.search_strategy = Some(Search::new(board));
    }

    fn calculate_best_move(&mut self) {
        let best_move: ChessMove = self.search_strategy.as_mut().unwrap().search();

        println!("bestmove {}", best_move.to_string());
    }
    
    fn init(&self) {
        println!("id name RustyFish");
        println!("id author Fernando Bandeira");
        println!("uciok");
    }
    
    fn is_ready(&self) {
        println!("readyok");
    }
    
    fn quit(&self) {
        println!("bye");
        std::process::exit(0);
    }
}
