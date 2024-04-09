use crate::{piece::Piece, position::Position};

#[derive(Debug, PartialEq)]
pub struct Board([Option<Piece>; 8 * 8]);

impl Default for Board {
    fn default() -> Self {
        Self([None; 8 * 8])
    }
}

impl Board {
    pub fn place_piece(&mut self, piece: Piece, position: &Position) {
        let index = position.board_index();
        *self.0.get_mut(index).expect(&format!(
            "Failed to place piece on board. The position is not in the correct range {:?}",
            position
        )) = Some(piece);
    }

    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        let index = position.board_index();
        self.0
            .get(index)
            .expect(&format!(
                "Failed to get piece. The position is not in the correct range {:?}",
                position
            ))
            .as_ref()
    }

    pub fn print_custom(&self, callback: impl Fn(Position) -> char) {
        println!("+---+---+---+---+---+---+---+---+");
        for i in 0..8 {
            print!("|");
            for j in 0..8 {
                print!(" {} |", callback(Position::new_unchecked(j, 7 - i)));
            }
            println!(" {}", 8 - i);
            println!("+---+---+---+---+---+---+---+---+");
        }
        println!("  a   b   c   d   e   f   g   h  ");
    }

    pub fn print_pieces(&self) {
        self.print_custom(|p| {
            let Some(piece) = self.get_piece(&p) else {
                return ' ';
            };
            match (piece.kind(), piece.color()) {
                (crate::piece_type::PieceType::Pawn, crate::color::Color::White) => 'P',
                (crate::piece_type::PieceType::Pawn, crate::color::Color::Black) => 'p',
                (crate::piece_type::PieceType::Knight, crate::color::Color::White) => 'N',
                (crate::piece_type::PieceType::Knight, crate::color::Color::Black) => 'n',
                (crate::piece_type::PieceType::Bishop, crate::color::Color::White) => 'B',
                (crate::piece_type::PieceType::Bishop, crate::color::Color::Black) => 'b',
                (crate::piece_type::PieceType::Rook, crate::color::Color::White) => 'R',
                (crate::piece_type::PieceType::Rook, crate::color::Color::Black) => 'r',
                (crate::piece_type::PieceType::Queen, crate::color::Color::White) => 'Q',
                (crate::piece_type::PieceType::Queen, crate::color::Color::Black) => 'q',
                (crate::piece_type::PieceType::King, crate::color::Color::White) => 'K',
                (crate::piece_type::PieceType::King, crate::color::Color::Black) => 'k',
            }
        });
    }
}
