use crate::{moves::Move, piece::Piece, position::Position};

#[derive(Debug, PartialEq)]
pub struct Board([Option<Piece>; 8 * 8]);

impl Default for Board {
    fn default() -> Self {
        Self([None; 8 * 8])
    }
}

impl Board {
    pub fn make_move(&mut self, from: &Position, to: &Position) {
        let move_piece = self.remove_piece(from);
        *self.piece_at_mut(to) = move_piece;
    }

    pub fn place_piece(&mut self, piece: Piece, position: &Position) {
        let index = position.board_index();
        *self.0.get_mut(index).unwrap_or_else(|| {
            panic!(
                "Failed to place piece on board. The position is not in the correct range {:?}",
                position
            )
        }) = Some(piece);
    }

    pub fn piece_at(&self, position: &Position) -> Option<&Piece> {
        let index = position.board_index();
        self.0
            .get(index)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get piece. The position is not in the correct range {:?}",
                    position
                )
            })
            .as_ref()
    }

    pub fn piece_at_mut(&mut self, position: &Position) -> &mut Option<Piece> {
        let index = position.board_index();
        self.0.get_mut(index).unwrap_or_else(|| {
            panic!(
                "Failed to get piece. The position is not in the correct range {:?}",
                position
            )
        })
    }

    pub fn remove_piece(&mut self, position: &Position) -> Option<Piece> {
        let index = position.board_index();
        self.0
            .get_mut(index)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to remove piece. The position is not in the correct range {:?}",
                    position
                )
            })
            .take()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
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

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn print_pieces(&self) {
        self.print_custom(
            // This is very ugly!!! But we take it for now.
            // Could probably improve this with a trait for the printer of a board
            #[cfg_attr(coverage_nightly, coverage(off))]
            |p| {
                let Some(piece) = self.piece_at(&p) else {
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
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, piece_type::PieceType};

    #[test]
    fn test_board_place_piece() {
        let mut board = Board::default();
        let pos = Position::new_unchecked(0, 0);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), &pos);
        assert_eq!(
            board.piece_at(&pos),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );
    }

    #[test]
    #[should_panic]
    fn place_with_invalid_position() {
        let mut board = Board::default();
        let pos = Position::new_unchecked(8, 8);
        board.place_piece(Piece::new(PieceType::Pawn, Color::White), &pos);
    }

    #[test]
    #[should_panic]
    fn get_piece_with_invalid_position() {
        let board = Board::default();
        let pos = Position::new_unchecked(8, 8);
        board.piece_at(&pos);
    }
}
