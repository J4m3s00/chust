use std::{fmt::Display, str::FromStr};

use crate::{piece_type::PieceType, position::Position};

#[derive(Debug, PartialEq, Clone)]
pub enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MoveType {
    Quiet,
    Capture(PieceType),
    EnPassantCapture,
    Castle,
    DoublePawnPush(Position),
    PromotionQuite(PromotionType),
    PromotionCapture(PromotionType, PieceType),
}

#[derive(Debug, PartialEq, Clone)]
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

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_type = match s.len() {
            4 => MoveType::Quiet,
            5 => {
                let promotion = PromotionType::from_str(&s[4..5])?;
                MoveType::PromotionQuite(promotion)
            }
            _ => return Err(anyhow::anyhow!("Invalid move string")),
        };
        let from = Position::from_str(&s[0..2])?;
        let to = Position::from_str(&s[2..4])?;
        Ok(Move::new(from, to, move_type))
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let promotion = match &self.move_type {
            MoveType::PromotionQuite(promotion) => format!("{}", promotion),
            MoveType::PromotionCapture(promotion, _) => format!("{}", promotion),
            _ => "".to_string(),
        };
        write!(f, "{}{}{}", self.from, self.to, promotion)
    }
}

impl Display for PromotionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let promotion = match self {
            PromotionType::Queen => "q",
            PromotionType::Rook => "r",
            PromotionType::Bishop => "b",
            PromotionType::Knight => "n",
        };
        write!(f, "{}", promotion)
    }
}

impl FromStr for PromotionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Q" | "q" => Ok(PromotionType::Queen),
            "R" | "r" => Ok(PromotionType::Rook),
            "B" | "b" => Ok(PromotionType::Bishop),
            "N" | "n" => Ok(PromotionType::Knight),
            _ => Err(anyhow::anyhow!("Invalid promotion type")),
        }
    }
}
