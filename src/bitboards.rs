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

    pub white_pinned: Bitboard,
    pub black_pinned: Bitboard,
}

impl GameBitBoards {
    pub fn new(game: &Game) -> Self {
        let mut this = Self::default();

        for (position, piece) in game.board().iter() {
            let bitboard = 1 << position.board_index();
            if let Some(piece) = piece {
                match (piece.kind(), piece.color()) {
                    (PieceType::Pawn, Color::White) => this.white_pawns |= bitboard,
                    (PieceType::Knight, Color::White) => this.white_knights |= bitboard,
                    (PieceType::Bishop, Color::White) => this.white_bishops |= bitboard,
                    (PieceType::Rook, Color::White) => this.white_rooks |= bitboard,
                    (PieceType::Queen, Color::White) => this.white_queens |= bitboard,
                    (PieceType::Pawn, Color::Black) => this.black_pawns |= bitboard,
                    (PieceType::Knight, Color::Black) => this.black_knights |= bitboard,
                    (PieceType::Bishop, Color::Black) => this.black_bishops |= bitboard,
                    (PieceType::Rook, Color::Black) => this.black_rooks |= bitboard,
                    (PieceType::Queen, Color::Black) => this.black_queens |= bitboard,
                    (PieceType::King, Color::White) => this.white_king = position,
                    (PieceType::King, Color::Black) => this.black_king = position,
                }
                match piece.color() {
                    Color::White => {
                        this.white_pieces |= bitboard;
                    }
                    Color::Black => {
                        this.black_pieces |= bitboard;
                    }
                }
            }
        }

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
        this.white_pinned = this.generate_king_pins(game, Color::White);
        this.black_pinned = this.generate_king_pins(game, Color::Black);

        this
    }

    fn generate_king_pins(&self, game: &Game, color: Color) -> Bitboard {
        let king_position = match color {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        };

        let mut res = Bitboard::default();

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

                if let Some(piece) = game.board().piece_at(&position) {
                    if piece.color() != color {
                        if count == 1 && direction.2.contains(&piece.kind()) {
                            res |= pinned;
                        }
                        break;
                    } else {
                        count += 1;
                    }
                }

                pinned |= 1 << position.board_index();
            }
        }

        res
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

    pub fn pinned(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pinned,
            Color::Black => self.black_pinned,
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
}

impl BitBoardPrinter {
    pub const ALL_IDENTIFIED: [(&'static str, Self); 18] = [
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
    ];
}

impl BoardPrinter for BitBoardPrinter {
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
            Self::WhitePinned => game.bitboards().white_pinned,
            Self::BlackPinned => game.bitboards().black_pinned,
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
