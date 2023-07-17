use std::{
    str::FromStr,
    sync::mpsc::Receiver,
};

use chess::{Board, Color, ChessMove, MoveGen};

use crate::utils::{MINUS_INFINITY, PLUS_INFINITY};
use crate::score;

pub struct Engine {
    board: Board,
    player_side: Color,
    enemy_side: Color,
    best_move: ChessMove,
    stop_receiver: Receiver<bool>,
    stop_search: bool,
}

impl Engine {
    pub fn new(stop_receiver: Receiver<bool>) -> Engine {
        Engine {
            board: Board::default(),
            player_side: Color::White,
            enemy_side: Color::Black,
            best_move: ChessMove::default(),
            stop_receiver,
            stop_search: false,
        }
    }

    // Example position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn position(&mut self, input: &str) {
        if !input.contains("fen") {
            return;
        }

        let fen_text: Option<&str> = input.split("fen").collect::<Vec<&str>>().pop();
        if fen_text.is_none() {
            return;
        }

        let board = Board::from_str(fen_text.unwrap().trim()).unwrap();
        self.board = board;

        let player_side: chess::Color = board.side_to_move();
        let enemy_side: chess::Color = match player_side {
            chess::Color::White => chess::Color::Black,
            chess::Color::Black => chess::Color::White,
        };
        self.player_side = player_side;
        self.enemy_side = enemy_side;
    }

    pub fn calculate_best_move(&mut self) {
        self.stop_search = false;
        let _ = self.negamax(self.board, 0, 10, MINUS_INFINITY, PLUS_INFINITY);
        println!("bestmove {}", self.best_move.to_string());
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
            if board.side_to_move() == self.player_side {
                return MINUS_INFINITY;
            }
            return PLUS_INFINITY;
        }
        if status == chess::BoardStatus::Stalemate {
            return 0;
        }

        if depth == 0 {
            return score::calculate_score(self.player_side, self.enemy_side, board);
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
            if self.stop_receiver.try_recv().is_ok() {
                self.stop_search = true;
                break;
            }
            if self.stop_search {
                break;
            }
        }
    
        return best_score;
    }
}
