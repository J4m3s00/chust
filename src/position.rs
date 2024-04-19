use std::{fmt::Display, str::FromStr};

use anyhow::Context;

/// The position on the board
/// Bottom left is (0, 0) or in chess terms 'A1'
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl Position {
    /// Creates a new position
    /// Returns None if the position is out of bounds
    pub fn new(x: u8, y: u8) -> Option<Self> {
        (x < 8 && y < 8).then_some(Self { x, y })
    }

    /// Creates a new position without bounds checking
    /// # WARN
    /// Must check, that x and y < 8
    pub fn new_unchecked(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Returns the x-y position as single index, for access in the [Board] array
    pub fn board_index(&self) -> usize {
        self.y as usize * 8 + self.x as usize
    }

    pub fn from_board_index(index: usize) -> Option<Self> {
        Self::new((index % 8) as u8, (index / 8) as u8)
    }

    /// Creates a new position without bounds checking
    /// # WARN
    /// Must check, that index < 64
    pub fn from_board_index_unchecked(index: usize) -> Self {
        Self::new_unchecked((index % 8) as u8, (index / 8) as u8)
    }

    pub fn offset(&self, x: i8, y: i8) -> Option<Self> {
        let new_x = self.x as i8 + x;
        let new_y = self.y as i8 + y;
        ((0..8).contains(&new_x) && (0..8).contains(&new_y)).then_some(Self {
            x: new_x as u8,
            y: new_y as u8,
        })
    }

    pub fn rank_direction(&self, other: &Self) -> i8 {
        let dx = other.x as i8 - self.x as i8;
        dx / dx.abs()
    }

    pub fn file_direction(&self, other: &Self) -> i8 {
        let dy = other.y as i8 - self.y as i8;
        dy / dy.abs()
    }

    pub fn file(&self) -> u8 {
        self.x
    }

    pub fn rank(&self) -> u8 {
        self.y
    }

    pub const A1: Self = Self { x: 0, y: 0 };
    pub const B1: Self = Self { x: 1, y: 0 };
    pub const C1: Self = Self { x: 2, y: 0 };
    pub const D1: Self = Self { x: 3, y: 0 };
    pub const E1: Self = Self { x: 4, y: 0 };
    pub const F1: Self = Self { x: 5, y: 0 };
    pub const G1: Self = Self { x: 6, y: 0 };
    pub const H1: Self = Self { x: 7, y: 0 };

    pub const A2: Self = Self { x: 0, y: 1 };
    pub const B2: Self = Self { x: 1, y: 1 };
    pub const C2: Self = Self { x: 2, y: 1 };
    pub const D2: Self = Self { x: 3, y: 1 };
    pub const E2: Self = Self { x: 4, y: 1 };
    pub const F2: Self = Self { x: 5, y: 1 };
    pub const G2: Self = Self { x: 6, y: 1 };
    pub const H2: Self = Self { x: 7, y: 1 };

    pub const A3: Self = Self { x: 0, y: 2 };
    pub const B3: Self = Self { x: 1, y: 2 };
    pub const C3: Self = Self { x: 2, y: 2 };
    pub const D3: Self = Self { x: 3, y: 2 };
    pub const E3: Self = Self { x: 4, y: 2 };
    pub const F3: Self = Self { x: 5, y: 2 };
    pub const G3: Self = Self { x: 6, y: 2 };
    pub const H3: Self = Self { x: 7, y: 2 };

    pub const A4: Self = Self { x: 0, y: 3 };
    pub const B4: Self = Self { x: 1, y: 3 };
    pub const C4: Self = Self { x: 2, y: 3 };
    pub const D4: Self = Self { x: 3, y: 3 };
    pub const E4: Self = Self { x: 4, y: 3 };
    pub const F4: Self = Self { x: 5, y: 3 };
    pub const G4: Self = Self { x: 6, y: 3 };
    pub const H4: Self = Self { x: 7, y: 3 };

    pub const A5: Self = Self { x: 0, y: 4 };
    pub const B5: Self = Self { x: 1, y: 4 };
    pub const C5: Self = Self { x: 2, y: 4 };
    pub const D5: Self = Self { x: 3, y: 4 };
    pub const E5: Self = Self { x: 4, y: 4 };
    pub const F5: Self = Self { x: 5, y: 4 };
    pub const G5: Self = Self { x: 6, y: 4 };
    pub const H5: Self = Self { x: 7, y: 4 };

    pub const A6: Self = Self { x: 0, y: 5 };
    pub const B6: Self = Self { x: 1, y: 5 };
    pub const C6: Self = Self { x: 2, y: 5 };
    pub const D6: Self = Self { x: 3, y: 5 };
    pub const E6: Self = Self { x: 4, y: 5 };
    pub const F6: Self = Self { x: 5, y: 5 };
    pub const G6: Self = Self { x: 6, y: 5 };
    pub const H6: Self = Self { x: 7, y: 5 };

    pub const A7: Self = Self { x: 0, y: 6 };
    pub const B7: Self = Self { x: 1, y: 6 };
    pub const C7: Self = Self { x: 2, y: 6 };
    pub const D7: Self = Self { x: 3, y: 6 };
    pub const E7: Self = Self { x: 4, y: 6 };
    pub const F7: Self = Self { x: 5, y: 6 };
    pub const G7: Self = Self { x: 6, y: 6 };
    pub const H7: Self = Self { x: 7, y: 6 };

    pub const A8: Self = Self { x: 0, y: 7 };
    pub const B8: Self = Self { x: 1, y: 7 };
    pub const C8: Self = Self { x: 2, y: 7 };
    pub const D8: Self = Self { x: 3, y: 7 };
    pub const E8: Self = Self { x: 4, y: 7 };
    pub const F8: Self = Self { x: 5, y: 7 };
    pub const G8: Self = Self { x: 6, y: 7 };
    pub const H8: Self = Self { x: 7, y: 7 };
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col_char = (b'a' + self.x) as char;
        let row_char = (b'1' + self.y) as char;
        write!(f, "{}{}", col_char, row_char)
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            anyhow::bail!("Invalid position string. Must be 2 characters long.");
        }
        let mut chars = s.chars();
        let col_char = chars.next().ok_or(anyhow::anyhow!("No column character"))?;
        let row_char = chars.next().ok_or(anyhow::anyhow!("No row character"))?;

        let x = (col_char as u8)
            .checked_sub(b'a')
            .with_context(|| format!("Unknown column char {col_char}"))?;
        let y = (row_char as u8)
            .checked_sub(b'1')
            .with_context(|| format!("Unknown row char {row_char}"))?;

        Self::new(x, y).ok_or(anyhow::anyhow!("Position out of bounds"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(0, 0);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert_eq!(pos.rank(), 0);
        assert_eq!(pos.file(), 0);
        let pos = Position::new(8, 8);
        assert!(pos.is_none());
    }

    #[test]
    fn test_position_board_index() {
        let pos = Position::new_unchecked(0, 0);
        assert_eq!(pos.board_index(), 0);
        let pos = Position::new_unchecked(7, 7);
        assert_eq!(pos.board_index(), 63);
    }

    #[test]
    fn test_position_display() {
        let pos = Position::new_unchecked(0, 0);
        assert_eq!(format!("{}", pos), "a1");
        let pos = Position::new_unchecked(7, 7);
        assert_eq!(format!("{}", pos), "h8");
    }

    #[test]
    fn test_position_from_str() {
        let pos = Position::from_str("a1");
        assert!(pos.is_ok());
        let pos = pos.unwrap();
        assert_eq!(pos.rank(), 0);
        assert_eq!(pos.file(), 0);
        let pos = Position::from_str("h8");
        assert!(pos.is_ok());
        let pos = pos.unwrap();
        assert_eq!(pos.rank(), 7);
        assert_eq!(pos.file(), 7);
        let pos = Position::from_str("i9");
        assert!(pos.is_err());
    }
}
