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
        let row_char = (b'a' + self.y) as char;
        let col_char = (b'1' + self.x) as char;
        write!(f, "{}{}", col_char, row_char)
    }
}
