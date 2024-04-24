use crate::moves::PromotionType;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 330,
            PieceType::Bishop => 320,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 1000,
        }
    }
}

impl From<&PromotionType> for PieceType {
    fn from(value: &PromotionType) -> Self {
        match value {
            PromotionType::Knight => PieceType::Knight,
            PromotionType::Bishop => PieceType::Bishop,
            PromotionType::Rook => PieceType::Rook,
            PromotionType::Queen => PieceType::Queen,
        }
    }
}
