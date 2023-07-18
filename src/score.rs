use chess::{BitBoard, Board, Color};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

pub fn calculate_score(board: Board) -> i32 {
    let player_side = board.side_to_move();
    let enemy_side: Color;
    if player_side == Color::White {
        enemy_side = Color::Black;
    } else {
        enemy_side = Color::White;
    }

    let player_score = score(player_side, board);
    let enemy_score = score(enemy_side, board);
    return player_score - enemy_score;
}

pub fn score(side: chess::Color, board: Board) -> i32 {
    let mut score: i32 = 0;

    let all_pieces: &BitBoard = board.color_combined(side);
    let pawns = all_pieces & board.pieces(chess::Piece::Pawn);
    let knights = all_pieces & board.pieces(chess::Piece::Knight);
    let bishops = all_pieces & board.pieces(chess::Piece::Bishop);
    let rooks = all_pieces & board.pieces(chess::Piece::Rook);
    let queens = all_pieces & board.pieces(chess::Piece::Queen);

    score += pawns.count() as i32 * PAWN_VALUE;
    score += knights.count() as i32 * KNIGHT_VALUE;
    score += bishops.count() as i32 * BISHOP_VALUE;
    score += rooks.count() as i32 * ROOK_VALUE;
    score += queens.count() as i32 * QUEEN_VALUE;

    return score;
}
