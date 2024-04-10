use crate::{piece_type::PieceType, position::Position};

#[derive(Debug, PartialEq)]
pub enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture(PieceType),
    Castle,
    EnPassant(Position),
    PromotionQuite(PromotionType),
    PromotionCapture(PromotionType, PieceType),
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(from: Position, to: Position, move_type: MoveType) -> Self {
        Self {
            from,
            to,
            move_type,
        }
    }
}
