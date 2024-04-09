use crate::{color::Color, piece_type::PieceType};

#[derive(Clone, Copy)]
pub struct Piece {
    kind: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn kind(&self) -> &PieceType {
        &self.kind
    }
}
