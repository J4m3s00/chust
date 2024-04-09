use std::fmt::Display;

/// The position on the board
/// Bottom left is (0, 0) or in chess terms 'A1'
#[derive(Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
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
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col_char = (b'a' + self.x) as char;
        let row_char = (b'1' + self.y) as char;
        write!(f, "{}{}", col_char, row_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(0, 0);
        assert!(pos.is_some());
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
}
