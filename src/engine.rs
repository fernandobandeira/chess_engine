use std::{str::FromStr, sync::mpsc::Receiver};

use chess::{Board, ChessMove, MoveGen};

use crate::score;
use crate::utils::{self, MINUS_INFINITY, PLUS_INFINITY};

pub struct Engine {
    board: Board,
    best_move: ChessMove,
    stop_receiver: Receiver<bool>,
    stop_search: bool,
}

impl Engine {
    pub fn new(stop_receiver: Receiver<bool>) -> Engine {
        Engine {
            board: Board::default(),
            best_move: ChessMove::default(),
            stop_receiver,
            stop_search: false,
        }
    }

    pub fn new_game(&mut self) {
        self.board = Board::default();
    }

    pub fn position(&mut self, position: &str) {
        // Example position startpos moves a2a3 b7b6
        if position.starts_with("position startpos") {
            self.board = Board::default();
        }

        if position.contains("moves") {
            // Loop through the moves
            for m in position.split(" ").skip(3) {
                let m = ChessMove::from_str(m.trim()).unwrap();
                self.board = self.board.make_move_new(m);
            }
        }
    }

    pub fn calculate_best_move(&mut self) {
        self.stop_search = false;

        let mut best_move_iter: ChessMove = ChessMove::default();
        let mut depth = 1;
        loop {
            let score = self.negamax(self.board, 0, depth, MINUS_INFINITY, PLUS_INFINITY);
            if self.stop_search {
                break;
            }
            log::debug!(
                "Depth: {} Score: {} Move: {}",
                depth,
                score,
                self.best_move.to_string()
            );
            depth += 1;
            best_move_iter = self.best_move;
        }

        utils::send_output(&format!("bestmove {}", best_move_iter.to_string()));
    }

    fn negamax(
        &mut self,
        board: Board,
        ply_from_root: u32,
        depth: u32,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        let status: chess::BoardStatus = board.status();
        if status == chess::BoardStatus::Checkmate {
            return MINUS_INFINITY;
        }
        if status == chess::BoardStatus::Stalemate {
            return 0;
        }

        if depth == 0 {
            return score::calculate_score(board);
        }

        let mut best_score = MINUS_INFINITY;
        let moves = MoveGen::new_legal(&board);
        for m in moves {
            let score = -self.negamax(
                board.make_move_new(m),
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
                    self.best_move = m;
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
