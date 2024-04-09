use crate::{board::Board, color::Color, fen::Fen};

pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    current_turn: Color,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Fen::parse_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Failed to parse default position.")
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
