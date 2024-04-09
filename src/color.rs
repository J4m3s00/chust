#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn inverse(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn pawn_rank(&self) -> u8 {
        match self {
            Self::White => 1,
            Self::Black => 6,
        }
    }

    pub fn root_rank(&self) -> u8 {
        match self {
            Self::White => 0,
            Self::Black => 7,
        }
    }

    pub fn board_direction(&self) -> i8 {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn inverse() {
        let white = Color::White;
        assert!(white.inverse() == Color::Black);
        let black = Color::Black;
        assert!(black.inverse() == Color::White);
    }

    #[test]
    fn ranks() {
        assert_eq!(Color::White.pawn_rank(), 1);
        assert_eq!(Color::Black.pawn_rank(), 6);
        assert_eq!(Color::White.root_rank(), 0);
        assert_eq!(Color::Black.root_rank(), 7);
        assert_eq!(Color::White.board_direction(), 1);
        assert_eq!(Color::Black.board_direction(), -1);
    }
}
