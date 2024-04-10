use crate::{piece::Piece, position::Position};

pub trait BoardPrinter {
    fn get_char(&self, piece: Option<Piece>, position: Position) -> char;
}

impl<F> BoardPrinter for F
where
    F: Fn(Option<Piece>, Position) -> char,
{
    fn get_char(&self, piece: Option<Piece>, position: Position) -> char {
        self(piece, position)
    }
}

pub struct DefaultBoardPrinter;

impl BoardPrinter for DefaultBoardPrinter {
    fn get_char(&self, piece: Option<Piece>, _: Position) -> char {
        let Some(piece) = piece else {
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
