use chess::{Board, ChessMove, MoveGen};
use super::utils::{MINUS_INFINITY, PLUS_INFINITY};

const DEPTH: i32 = 4;

pub struct Search {
    board: Board,
    player_side: chess::Color,
    enemy_side: chess::Color,
    best_move: ChessMove,
}

impl Default for Search {
    fn default() -> Self {
        Search {
            board: Board::default(),
            player_side: chess::Color::White,
            enemy_side: chess::Color::Black,
            best_move: ChessMove::default(),
        }
    }
}

impl Search {
    pub fn new(board: Board) -> Self {
        let player_side: chess::Color = board.side_to_move();
        let enemy_side: chess::Color = match player_side {
            chess::Color::White => chess::Color::Black,
            chess::Color::Black => chess::Color::White,
        };

        Search {
            board,
            player_side,
            enemy_side,
            best_move: ChessMove::default(),
        }
    }

    pub fn search(&mut self) -> ChessMove {
        let _ = self.negamax(self.board, 0, DEPTH, MINUS_INFINITY, PLUS_INFINITY);
        return self.best_move;
    }

    fn negamax(
        &mut self,
        board: Board,
        ply_from_root: i32,
        depth: i32,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        if depth == 0 {
            return super::score::calculate_score(self.player_side, self.enemy_side, board);
        }
    
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
        }
    
        return best_score;
    }
}
