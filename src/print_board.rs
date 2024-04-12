use crate::{game::Game, position::Position};

pub trait BoardPrinter {
    fn get_char(&self, position: Position, game: &Game) -> char;
}

impl<F> BoardPrinter for F
where
    F: Fn(Position, &Game) -> char,
{
    fn get_char(&self, position: Position, game: &Game) -> char {
        self(position, game)
    }
}

pub struct DefaultBoardPrinter;

impl BoardPrinter for DefaultBoardPrinter {
    fn get_char(&self, position: Position, game: &Game) -> char {
        let Some(piece) = game.board().piece_at(&position) else {
            return ' ';
        };
        match (piece.kind(), piece.color()) {
            (crate::piece_type::PieceType::Pawn, crate::color::Color::White) => 'P',
            (crate::piece_type::PieceType::Pawn, crate::color::Color::Black) => 'p',
            (crate::piece_type::PieceType::Knight, crate::color::Color::White) => 'N',
            (crate::piece_type::PieceType::Knight, crate::color::Color::Black) => 'n',
            (crate::piece_type::PieceType::Bishop, crate::color::Color::White) => 'B',
            (crate::piece_type::PieceType::Bishop, crate::color::Color::Black) => 'b',
            (crate::piece_type::PieceType::Rook, crate::color::Color::White) => 'R',
            (crate::piece_type::PieceType::Rook, crate::color::Color::Black) => 'r',
            (crate::piece_type::PieceType::Queen, crate::color::Color::White) => 'Q',
            (crate::piece_type::PieceType::Queen, crate::color::Color::Black) => 'q',
            (crate::piece_type::PieceType::King, crate::color::Color::White) => 'K',
            (crate::piece_type::PieceType::King, crate::color::Color::Black) => 'k',
        }
    }
}
