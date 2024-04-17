use crate::{color::Color, piece_type::PieceType};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    kind: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece_type(&self) -> PieceType {
        self.kind
    }

    pub fn get_print_char(&self) -> char {
        match (self.piece_type(), self.color()) {
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
