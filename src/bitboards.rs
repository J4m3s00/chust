use crate::{color::Color, game::Game, piece_type::PieceType, print_board::BoardPrinter};

#[derive(Default, Debug, PartialEq)]
pub struct GameBitBoards {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
    pub all_pieces: u64,
}

impl GameBitBoards {
    pub fn new(game: &Game) -> Self {
        let mut this = Self::default();

        for (position, piece) in game.board().iter() {
            let bitboard = 1 << position.board_index();
            match piece {
                Some(piece) => match (piece.kind(), piece.color()) {
                    (PieceType::Pawn, Color::White) => this.white_pawns |= bitboard,
                    (PieceType::Knight, Color::White) => this.white_knights |= bitboard,
                    (PieceType::Bishop, Color::White) => this.white_bishops |= bitboard,
                    (PieceType::Rook, Color::White) => this.white_rooks |= bitboard,
                    (PieceType::Queen, Color::White) => this.white_queens |= bitboard,
                    (PieceType::King, Color::White) => this.white_king |= bitboard,
                    (PieceType::Pawn, Color::Black) => this.black_pawns |= bitboard,
                    (PieceType::Knight, Color::Black) => this.black_knights |= bitboard,
                    (PieceType::Bishop, Color::Black) => this.black_bishops |= bitboard,
                    (PieceType::Rook, Color::Black) => this.black_rooks |= bitboard,
                    (PieceType::Queen, Color::Black) => this.black_queens |= bitboard,
                    (PieceType::King, Color::Black) => this.black_king |= bitboard,
                },
                None => {}
            }
            this.all_pieces |= bitboard;
        }

        this
    }
}




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
}

impl BoardPrinter for BitBoardPrinter {
    fn get_char(
        &self,
        position: crate::position::Position,
        game: &Game,
    ) -> char {
        let bit_value = match self {
            Self::WhitePawns => 
        };

    }
}
