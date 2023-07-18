use shakmaty::{Chess, Position, Role};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

pub fn calculate_score(board: Chess) -> i32 {
    let mut score: i32 = 0;

    // Add up the score for each piece that we have
    score += board.our(Role::Pawn).count() as i32 * PAWN_VALUE;
    score += board.our(Role::Knight).count() as i32 * KNIGHT_VALUE;
    score += board.our(Role::Bishop).count() as i32 * BISHOP_VALUE;
    score += board.our(Role::Rook).count() as i32 * ROOK_VALUE;
    score += board.our(Role::Queen).count() as i32 * QUEEN_VALUE;

    // Subtract the score for each piece that the opponent has
    score -= board.their(Role::Pawn).count() as i32 * PAWN_VALUE;
    score -= board.their(Role::Knight).count() as i32 * KNIGHT_VALUE;
    score -= board.their(Role::Bishop).count() as i32 * BISHOP_VALUE;
    score -= board.their(Role::Rook).count() as i32 * ROOK_VALUE;
    score -= board.their(Role::Queen).count() as i32 * QUEEN_VALUE;

    return score;
}
