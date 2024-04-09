use crate::{color::Color, piece::Piece};

pub struct Game {
    current_turn: Color,
    board: [Piece; 8 * 8],
}
