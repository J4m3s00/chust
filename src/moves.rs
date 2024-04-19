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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_move_from_str() {
        use super::{Move, MoveType, Position};
        let move_str = "e2e4";
        let expected_move = Move::new(Position::E2, Position::E4, MoveType::Quiet);
        assert_eq!(Move::from_str(move_str).unwrap(), expected_move);

        let move_str = "e7e8q";
        let expected_move = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Queen),
        );
        assert_eq!(Move::from_str(move_str).unwrap(), expected_move);

        let move_str = "e7e8r";
        let expected_move = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Rook),
        );
        assert_eq!(Move::from_str(move_str).unwrap(), expected_move);

        let move_str = "e7e8b";
        let expected_move = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Bishop),
        );
        assert_eq!(Move::from_str(move_str).unwrap(), expected_move);

        let move_str = "e7e8n";
        let expected_move = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Knight),
        );
        assert_eq!(Move::from_str(move_str).unwrap(), expected_move);

        assert!("e7e8j".parse::<Move>().is_err());
        assert!("asgldf".parse::<Move>().is_err());
    }

    #[test]
    fn test_move_to_string() {
        use super::{Move, MoveType, Position};
        let mov = Move::new(Position::E2, Position::E4, MoveType::Quiet);
        assert_eq!(mov.to_string(), "e2e4");

        let mov = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Queen),
        );
        assert_eq!(mov.to_string(), "e7e8q");

        let mov = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Rook),
        );
        assert_eq!(mov.to_string(), "e7e8r");

        let mov = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Bishop),
        );
        assert_eq!(mov.to_string(), "e7e8b");

        let mov = Move::new(
            Position::E7,
            Position::E8,
            MoveType::PromotionQuite(super::PromotionType::Knight),
        );
        assert_eq!(mov.to_string(), "e7e8n");
    }
}
