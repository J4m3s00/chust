use crate::{board::Board, color::Color, fen::Fen};

pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both,
}

pub struct Game {
    current_turn: Color,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            current_turn: Color::White,
        }
    }
}

impl Game {
    pub fn new(board: Board, current_turn: Color) -> Self {
        Self {
            board,
            current_turn,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_turn(&self) -> Color {
        self.current_turn
    }
}
