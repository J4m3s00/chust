use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::{
    color::Color, game::Game, move_generation::MoveGenerator, piece_type::PieceType,
    position::Position, print_board::BoardPrinter,
};

#[derive(Default, Debug, PartialEq)]
pub struct GameBitBoards {
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_pieces: Bitboard,
    pub white_pieces: Bitboard,

    pub white_king: Position,
    pub black_king: Position,

    pub white_attacks: Bitboard,
    pub black_attacks: Bitboard,

    pub white_pinned: Vec<Bitboard>,
    pub black_pinned: Vec<Bitboard>,

    pub white_blockable_check: Vec<Bitboard>,
    pub black_blockable_check: Vec<Bitboard>,
}

impl GameBitBoards {
    pub fn new(game: &Game) -> Self {
        let mut this = Self::default();
        this.generate_pieces(game);

        // Possible attacks
        let move_generator = MoveGenerator::new(game);
        for position in this.white_pieces.iter() {
            for attack in move_generator.possible_attacking_moves(&position) {
                this.white_attacks |= 1 << attack.to.board_index();
            }
        }
        for position in this.black_pieces.iter() {
            for attack in move_generator.possible_attacking_moves(&position) {
                this.black_attacks |= 1 << attack.to.board_index();
            }
        }

        // Pins
        this.generate_king_pins_and_checks(game, Color::White);
        this.generate_king_pins_and_checks(game, Color::Black);

        this
    }

    fn generate_pieces(&mut self, game: &Game) {
        for (position, piece) in game.board().iter() {
            let bitboard = 1 << position.board_index();
            if let Some(piece) = piece {
                match (piece.piece_type(), piece.color()) {
                    (PieceType::Pawn, Color::White) => self.white_pawns |= bitboard,
                    (PieceType::Knight, Color::White) => self.white_knights |= bitboard,
                    (PieceType::Bishop, Color::White) => self.white_bishops |= bitboard,
                    (PieceType::Rook, Color::White) => self.white_rooks |= bitboard,
                    (PieceType::Queen, Color::White) => self.white_queens |= bitboard,
                    (PieceType::Pawn, Color::Black) => self.black_pawns |= bitboard,
                    (PieceType::Knight, Color::Black) => self.black_knights |= bitboard,
                    (PieceType::Bishop, Color::Black) => self.black_bishops |= bitboard,
                    (PieceType::Rook, Color::Black) => self.black_rooks |= bitboard,
                    (PieceType::Queen, Color::Black) => self.black_queens |= bitboard,
                    (PieceType::King, Color::White) => self.white_king = position,
                    (PieceType::King, Color::Black) => self.black_king = position,
                }
                match piece.color() {
                    Color::White => {
                        self.white_pieces |= bitboard;
                    }
                    Color::Black => {
                        self.black_pieces |= bitboard;
                    }
                }
            }
        }
    }

    // Needs to be genrated with a valid pieces bitboard
    fn generate_king_pins_and_checks(&mut self, game: &Game, color: Color) {
        let king_position = match color {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        };

        let mut pins = Vec::default();
        let mut checks = Vec::default();

        for direction in &[
            (1, 0, &[PieceType::Rook, PieceType::Queen]),
            (1, 1, &[PieceType::Bishop, PieceType::Queen]),
            (0, 1, &[PieceType::Rook, PieceType::Queen]),
            (-1, 1, &[PieceType::Bishop, PieceType::Queen]),
            (-1, 0, &[PieceType::Rook, PieceType::Queen]),
            (-1, -1, &[PieceType::Bishop, PieceType::Queen]),
            (0, -1, &[PieceType::Rook, PieceType::Queen]),
            (1, -1, &[PieceType::Bishop, PieceType::Queen]),
        ] {
            let mut position = king_position;
            let mut count = 0;
            let mut pinned = Bitboard::default();
            loop {
                position = match position.offset(direction.0, direction.1) {
                    Some(position) => position,
                    None => break,
                };
                pinned |= 1 << position.board_index();

                if let Some(piece) = game.board().piece_at(&position) {
                    if piece.color() != color {
                        if direction.2.contains(&piece.piece_type()) {
                            if count == 0 {
                                checks.push(pinned);
                            } else if count == 1 {
                                pins.push(pinned);
                            } else {
                                unreachable!()
                            }
                        }
                        break;
                    } else {
                        count += 1;
                    }
                }

                if count >= 2 {
                    break; // We have two pieces in between. No checks or pins can occur
                }
            }
        }

        // Find pawn and knight checks as well.
        let move_generator = MoveGenerator::new(game);
        for piece_position in self
            .knights(color.opposite())
            .iter()
            .chain(self.pawns(color.opposite()).iter())
        {
            for attack in move_generator.possible_attacking_moves(&piece_position) {
                if attack.to == king_position {
                    checks.push(Bitboard::from(1 << attack.from.board_index()));
                }
            }
        }

        match color {
            Color::White => {
                self.white_pinned = pins;
                self.white_blockable_check = checks;
            }
            Color::Black => {
                self.black_pinned = pins;
                self.black_blockable_check = checks;
            }
        }
    }

    // Helper functions to get correct bitboard
    pub fn pawns(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pawns,
            Color::Black => self.black_pawns,
        }
    }

    pub fn knights(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_knights,
            Color::Black => self.black_knights,
        }
    }

    pub fn bishops(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_bishops,
            Color::Black => self.black_bishops,
        }
    }

    pub fn rooks(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_rooks,
            Color::Black => self.black_rooks,
        }
    }

    pub fn queens(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_queens,
            Color::Black => self.black_queens,
        }
    }

    pub fn king(&self, color: Color) -> Position {
        match color {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        }
    }

    pub fn pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        }
    }

    pub fn attacks(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_attacks,
            Color::Black => self.black_attacks,
        }
    }

    pub fn pinned(&self, color: Color) -> &[Bitboard] {
        match color {
            Color::White => &self.white_pinned,
            Color::Black => &self.black_pinned,
        }
    }

    pub fn blockable_checks(&self, color: Color) -> &[Bitboard] {
        match color {
            Color::White => &self.white_blockable_check,
            Color::Black => &self.black_blockable_check,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BitBoardPrinter {
    WhitePawns,
    WhiteKnights,
    WhiteBishops,
    WhiteRooks,
    WhiteQueens,
    WhiteKing,
    BlackPawns,
    BlackKnights,
    BlackBishops,
    BlackRooks,
    BlackQueens,
    BlackKing,

    WhitePieces,
    BlackPieces,

    WhiteAttacks,
    BlackAttacks,

    WhitePinned,
    BlackPinned,

    WhiteBlockableChecks,
    BlackBlockableChecks,
}

impl BitBoardPrinter {
    pub const ALL_IDENTIFIED: [(&'static str, Self); 20] = [
        ("white_pawns", Self::WhitePawns),
        ("white_knights", Self::WhiteKnights),
        ("white_bishops", Self::WhiteBishops),
        ("white_rooks", Self::WhiteRooks),
        ("white_queens", Self::WhiteQueens),
        ("white_king", Self::WhiteKing),
        ("black_pawns", Self::BlackPawns),
        ("black_knights", Self::BlackKnights),
        ("black_bishops", Self::BlackBishops),
        ("black_rooks", Self::BlackRooks),
        ("black_queens", Self::BlackQueens),
        ("black_king", Self::BlackKing),
        ("white_pieces", Self::WhitePieces),
        ("black_pieces", Self::BlackPieces),
        ("white_attacks", Self::WhiteAttacks),
        ("black_attacks", Self::BlackAttacks),
        ("white_pinned", Self::WhitePinned),
        ("black_pinned", Self::BlackPinned),
        ("white_checks", Self::WhiteBlockableChecks),
        ("black_checks", Self::BlackBlockableChecks),
    ];
}

impl BoardPrinter for BitBoardPrinter {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn get_char(&self, position: crate::position::Position, game: &Game) -> char {
        let bit_value = match self {
            Self::WhitePawns => game.bitboards().white_pawns,
            Self::WhiteKnights => game.bitboards().white_knights,
            Self::WhiteBishops => game.bitboards().white_bishops,
            Self::WhiteRooks => game.bitboards().white_rooks,
            Self::WhiteQueens => game.bitboards().white_queens,
            Self::WhiteKing => Bitboard::from(game.bitboards().white_king),
            Self::BlackPawns => game.bitboards().black_pawns,
            Self::BlackKnights => game.bitboards().black_knights,
            Self::BlackBishops => game.bitboards().black_bishops,
            Self::BlackRooks => game.bitboards().black_rooks,
            Self::BlackQueens => game.bitboards().black_queens,
            Self::BlackKing => Bitboard::from(game.bitboards().black_king),
            Self::WhitePieces => game.bitboards().white_pieces,
            Self::BlackPieces => game.bitboards().black_pieces,
            Self::WhiteAttacks => game.bitboards().white_attacks,
            Self::BlackAttacks => game.bitboards().black_attacks,
            Self::WhitePinned => game
                .bitboards()
                .white_pinned
                .iter()
                .fold(Bitboard::default(), |acc, pinned| acc | *pinned),
            Self::BlackPinned => game
                .bitboards()
                .black_pinned
                .iter()
                .fold(Bitboard::default(), |acc, pinned| acc | *pinned),
            Self::WhiteBlockableChecks => game
                .bitboards()
                .white_blockable_check
                .iter()
                .fold(Bitboard::default(), |acc, pinned| acc | *pinned),
            Self::BlackBlockableChecks => game
                .bitboards()
                .black_blockable_check
                .iter()
                .fold(Bitboard::default(), |acc, pinned| acc | *pinned),
        };

        let is_occupied = bit_value & (1 << position.board_index()) != 0;
        if is_occupied {
            'X'
        } else {
            ' '
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn iter(&self) -> impl Iterator<Item = Position> + '_ {
        (0..64).filter_map(|index| {
            (self.0 & (1 << index) != 0).then_some(Position::from_board_index_unchecked(index))
        })
    }

    pub fn contains(&self, position: &Position) -> bool {
        self.0 & (1 << position.board_index()) != 0
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Position> for Bitboard {
    fn from(position: Position) -> Self {
        Self(1 << position.board_index())
    }
}

impl BitOr<u64> for Bitboard {
    type Output = u64;

    fn bitor(self, rhs: u64) -> Self::Output {
        self.0 | rhs
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from(self.0 | rhs.0)
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

impl BitAnd<Bitboard> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from(self.0 & rhs.0)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = u64;

    fn not(self) -> Self::Output {
        !self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::fen::Fen;

    use super::*;

    #[test]
    fn test_iter() {
        let bitboard = Bitboard(0b1010101010101010);
        let positions: Vec<Position> = bitboard.iter().collect();
        assert_eq!(positions.len(), 8);
        assert!(positions.contains(&Position::from_board_index_unchecked(1)));
        assert!(positions.contains(&Position::from_board_index_unchecked(3)));
        assert!(positions.contains(&Position::from_board_index_unchecked(5)));
        assert!(positions.contains(&Position::from_board_index_unchecked(7)));
        assert!(positions.contains(&Position::from_board_index_unchecked(9)));
        assert!(positions.contains(&Position::from_board_index_unchecked(11)));
        assert!(positions.contains(&Position::from_board_index_unchecked(13)));
        assert!(positions.contains(&Position::from_board_index_unchecked(15)));
    }

    #[test]
    fn test_contains() {
        let bitboard = Bitboard(0b1010101010101010);
        assert!(bitboard.contains(&Position::from_board_index_unchecked(1)));
        assert!(!bitboard.contains(&Position::from_board_index_unchecked(2)));
        assert!(bitboard.contains(&Position::from_board_index_unchecked(3)));
        assert!(!bitboard.contains(&Position::from_board_index_unchecked(4)));
        assert!(bitboard.contains(&Position::from_board_index_unchecked(5)));
        assert!(!bitboard.contains(&Position::from_board_index_unchecked(6)));
        assert!(bitboard.contains(&Position::from_board_index_unchecked(7)));
        assert!(!bitboard.contains(&Position::from_board_index_unchecked(8)));
    }

    #[test]
    fn test_inner() {
        let bitboard = Bitboard(0b1010101010101010);
        assert_eq!(bitboard.inner(), 0b1010101010101010);
    }

    #[test]
    fn test_from_u64() {
        let bitboard: Bitboard = 0b1010101010101010.into();
        assert_eq!(bitboard.inner(), 0b1010101010101010);
    }

    #[test]
    fn test_from_position() {
        let position = Position::from_board_index_unchecked(3);
        let bitboard: Bitboard = position.into();
        assert_eq!(bitboard.inner(), 0b1000);
    }

    #[test]
    fn test_bitor_u64() {
        let bitboard = Bitboard(0b1010101010101010);
        let result = bitboard.bitor(0b0101010101010101);
        assert_eq!(result, 0b1111111111111111);
    }

    #[test]
    fn test_bitor_bitboard() {
        let bitboard1 = Bitboard(0b1010101010101010);
        let bitboard2 = Bitboard(0b0101010101010101);
        let result = bitboard1.bitor(bitboard2);
        assert_eq!(result.inner(), 0b1111111111111111);
    }

    #[test]
    fn test_bitor_assign_u64() {
        let mut bitboard = Bitboard(0b1010101010101010);
        bitboard.bitor_assign(0b0101010101010101);
        assert_eq!(bitboard.inner(), 0b1111111111111111);
    }

    #[test]
    fn test_bitor_assign_bitboard() {
        let mut bitboard1 = Bitboard(0b1010101010101010);
        let bitboard2 = Bitboard(0b0101010101010101);
        bitboard1.bitor_assign(bitboard2);
        assert_eq!(bitboard1.inner(), 0b1111111111111111);
    }

    #[test]
    fn test_bitand_u64() {
        let bitboard = Bitboard(0b1010101010101010);
        let result = bitboard.bitand(0b0101010101010101);
        assert_eq!(result, 0b0);
    }

    #[test]
    fn test_bitand_bitboard() {
        let bitboard1 = Bitboard(0b1010101010101010);
        let bitboard2 = Bitboard(0b0101010101010101);
        let result = bitboard1.bitand(bitboard2);
        assert_eq!(result.inner(), 0b0);
    }

    #[test]
    fn test_bitand_assign_u64() {
        let mut bitboard = Bitboard(0b1010101010101010);
        bitboard.bitand_assign(0b0101010101010101);
        assert_eq!(bitboard.inner(), 0b0);
    }

    #[test]
    fn test_bitand_assign_bitboard() {
        let mut bitboard1 = Bitboard(0b1010101010101010);
        let bitboard2 = Bitboard(0b0101010101010101);
        bitboard1.bitand_assign(bitboard2);
        assert_eq!(bitboard1.inner(), 0b0);
    }

    #[test]
    fn test_not() {
        let bitboard = Bitboard(0b1010101010101010);
        let result = !bitboard;
        assert_eq!(
            result,
            0b1111111111111111111111111111111111111111111111110101010101010101
        );
    }

    #[test]
    fn test_the_pieces() {
        let game =
            Fen::parse_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let bitboards = GameBitBoards::new(&game);

        assert_eq!(bitboards.pawns(Color::White).inner(), 0b1111111100000000);
        assert_eq!(bitboards.knights(Color::White).inner(), 0b01000010);
        assert_eq!(bitboards.bishops(Color::White).inner(), 0b00100100);
        assert_eq!(bitboards.rooks(Color::White).inner(), 0b10000001);
        assert_eq!(bitboards.queens(Color::White).inner(), 0b00001000);
        assert_eq!(bitboards.king(Color::White), Position::E1);

        assert_eq!(bitboards.pawns(Color::Black).inner(), 0b11111111 << 48);
        assert_eq!(bitboards.knights(Color::Black).inner(), 0b01000010 << 56);
        assert_eq!(bitboards.bishops(Color::Black).inner(), 0b00100100 << 56);
        assert_eq!(bitboards.rooks(Color::Black).inner(), 0b10000001 << 56);
        assert_eq!(bitboards.queens(Color::Black).inner(), 0b00001000 << 56);
        assert_eq!(bitboards.king(Color::Black), Position::E8);
    }
}
