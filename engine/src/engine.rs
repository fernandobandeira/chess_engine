use std::{str::FromStr, sync::mpsc::Receiver};

use shakmaty::CastlingMode;
use shakmaty::{Chess, Position, uci::Uci, Move};

use crate::score;
use crate::utils::{self, MINUS_INFINITY, PLUS_INFINITY};

pub struct Engine {
    board: Chess,
    best_move: Option<Move>,
    stop_receiver: Receiver<bool>,
    stop_search: bool,
}

impl Engine {
    pub fn new(stop_receiver: Receiver<bool>) -> Engine {
        Engine {
            board: Chess::default(),
            best_move: None,
            stop_receiver,
            stop_search: false,
        }
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

    pub fn calculate_best_move(&mut self) {
        self.stop_search = false;

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
