use std::{str::FromStr, sync::mpsc::Receiver, collections::HashMap};

use rand::Rng;
use shakmaty::san::San;
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use shakmaty::{CastlingMode, EnPassantMode};
use shakmaty::{Chess, Position, uci::Uci, Move};

use crate::score;
use crate::utils::{self, MINUS_INFINITY, PLUS_INFINITY};

pub struct Engine {
    board: Chess,
    best_move: Option<Move>,
    stop_receiver: Receiver<bool>,
    stop_search: bool,
    white_book: HashMap<u64, Vec<String>>,
    black_book: HashMap<u64, Vec<String>>,
    draw_book: HashMap<u64, Vec<String>>,
    out_of_book: bool,
}

impl Engine {
    pub fn new(stop_receiver: Receiver<bool>) -> Engine {
        let white_book = include_str!("../book/white_wins_moves.bin").to_string();
        let black_book = include_str!("../book/black_wins_moves.bin").to_string();
        let draw_book = include_str!("../book/draw_moves.bin").to_string();

        let engine = Engine {
            board: Chess::default(),
            best_move: None,
            stop_receiver,
            stop_search: false,
            white_book: Engine::prepare_book(white_book),
            black_book: Engine::prepare_book(black_book),
            draw_book: Engine::prepare_book(draw_book),
            out_of_book: false,
        };

        return engine;
    }

    pub fn prepare_book(book: String) -> HashMap<u64, Vec<String>> {
        let mut hash_map: HashMap<u64, Vec<String>> = HashMap::new();

        let book: Vec<&str> = book.split("\n").collect();
        for line in book {
            if line == "" {
                continue;
            }

            // Break line into hash and count
            let line: Vec<&str> = line.split(" ").collect();

            let hash = line[0];

            // Add the hash to the white book
            let hash: u64 = hash.parse().unwrap();
            let moves: Vec<String> = line[1..].iter().map(|s| s.to_string()).collect();
            
            hash_map.insert(hash, moves);
        }

        return hash_map;
    }

    pub fn new_game(&mut self) {
        self.board = Chess::default();
    }

    pub fn position(&mut self, position: &str) {
        // Example position startpos moves a2a3 b7b6
        if position.starts_with("position startpos") {
            self.board = Chess::default();
        }

        if position.contains("moves") {
            // Loop through the moves
            for m in position.split(" ").skip(3) {
                // Parse the move using shakmaty
                let uci: Uci = Uci::from_str(m.trim()).unwrap();
                let new_move = uci.to_move(&self.board).unwrap();
                self.board.play_unchecked(&new_move);
            }
        }
    }

    fn get_book_move(&mut self) -> Option<Move> {
        let turn = self.board.turn();

        let zobrist: Zobrist64 = self.board.zobrist_hash::<Zobrist64>(EnPassantMode::Legal);
        let hash: u64 = zobrist.0;
        let mut moves: Option<&Vec<String>> = None;

        if turn.is_white() {
            moves = self.white_book.get(&hash);
        }
        if turn.is_black() {
            moves = self.black_book.get(&hash);
        }
        if moves == None {
            moves = self.draw_book.get(&hash);
        }

        if moves != None {
            let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
            let moves: &Vec<String> = moves.unwrap();

            let random_index: usize = rng.gen_range(0..moves.len());
            let random_move: String = moves[random_index].clone();

            let best_move: Move = San::from_str(&random_move).unwrap().to_move(&self.board).unwrap();
            return Some(best_move);
        }

        // If we get here, the hash was not found
        self.out_of_book = true;
        return None;
    }

    pub fn calculate_best_move(&mut self) {
        self.stop_search = false;

        // Check if we are out of book
        if !self.out_of_book {
            let best_move: Option<Move> = self.get_book_move();
            if best_move != None {
                utils::send_output(&format!("bestmove {}", best_move.unwrap().to_uci(CastlingMode::Standard).to_string()));
                return;
            }
        }

        let mut best_move_iter: Option<Move> = None;
        let mut depth = 1;
        loop {
            let board = self.board.clone();
            let score = self.negamax(board, 0, depth, MINUS_INFINITY, PLUS_INFINITY);
            if self.stop_search {
                break;
            }
            log::debug!(
                "Depth: {} Score: {} Move: {}",
                depth,
                score,
                self.best_move.as_ref().unwrap().to_string()
            );
            depth += 1;
            best_move_iter = Some(self.best_move.as_ref().unwrap().clone());
        }

        // Check if best_move_iter is initialized
        if best_move_iter == None {
            return;
        }
        utils::send_output(&format!("bestmove {}", best_move_iter.unwrap().to_uci(CastlingMode::Standard).to_string()));
    }

    fn negamax(
        &mut self,
        board: Chess,
        ply_from_root: u32,
        depth: u32,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        if board.is_checkmate() {
            return MINUS_INFINITY;
        }
        if board.is_stalemate() || board.is_insufficient_material() {
            return 0;
        }

        if depth == 0 {
            return score::calculate_score(board);
        }

        let mut best_score = MINUS_INFINITY;
        let moves = board.legal_moves();
        for m in moves {
            let mut new_board = board.clone();
            new_board.play_unchecked(&m);

            let score = -self.negamax(
                new_board,
                ply_from_root + 1,
                depth - 1,
                -beta,
                -alpha,
            );

            best_score = best_score.max(score);

            // New best move
            if best_score > alpha {
                alpha = best_score;
                if ply_from_root == 0 {
                    self.best_move = Some(m);
                }
            }

            // This move is too good for us, so opponent won't choose it
            if alpha >= beta {
                return alpha;
            }

            // Stop searching if requested
            if self.stop_search {
                break;
            }

            if self.stop_receiver.try_recv().is_ok() {
                self.stop_search = true;
                break;
            }
        }

        return best_score;
    }
}
