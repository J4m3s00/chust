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
}
