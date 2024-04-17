use crate::{piece::Piece, position::Position};

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

    pub fn iter(&self) -> impl Iterator<Item = (Position, Option<&Piece>)> {
        self.0.iter().enumerate().map(|(i, piece)| {
            let position = Position::from_board_index(i).unwrap();
            (position, piece.as_ref())
        })
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
