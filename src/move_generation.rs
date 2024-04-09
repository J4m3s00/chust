use crate::{board::Board, color::Color, moves::Move, piece_type::PieceType, position::Position};

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn pseudo_legal_moves(board: &Board, position: &Position) -> Vec<Move> {
        let Some(piece) = board.piece_at(position) else {
            return Vec::new();
        };

        match piece.kind() {
            PieceType::Pawn => Self::pawn_pseudo_legal_moves(board, position, piece.color()),
            PieceType::Knight => Self::knight_pseudo_legal_moves(board, position, piece.color()),
            PieceType::Bishop => Self::bishop_pseudo_legal_moves(board, position, piece.color()),
            PieceType::Rook => Self::rook_pseudo_legal_moves(board, position, piece.color()),
            PieceType::Queen => Self::queen_pseudo_legal_moves(board, position, piece.color()),
            PieceType::King => Self::king_pseudo_legal_moves(board, position, piece.color()),
        }
    }

    fn pawn_pseudo_legal_moves(board: &Board, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        let direction = color.board_direction();

        // Single step forward
        if let Some(new_pos) = position.offset(0, direction) {
            if board.piece_at(&new_pos).is_none() {
                result.push(Move::new(*position, new_pos));
            }
        }

        // Double step forward
        if position.rank() == color.pawn_rank() {
            if let Some(new_pos) = position.offset(0, 2 * direction) {
                if board.piece_at(&new_pos).is_none() {
                    result.push(Move::new(*position, new_pos));
                }
            }
        }

        // Capture moves
        for &dx in &[-1, 1] {
            if let Some(new_pos) = position.offset(dx, direction) {
                if let Some(piece) = board.piece_at(&new_pos) {
                    if piece.color() != color {
                        result.push(Move::new(*position, new_pos));
                    }
                }
            }
        }

        result
    }

    fn knight_pseudo_legal_moves(board: &Board, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        for &dx in &[-2i8, -1, 1, 2] {
            for &dy in &[-2i8, -1, 1, 2] {
                if dx.abs() != dy.abs() {
                    if let Some(new_pos) = position.offset(dx, dy) {
                        if let Some(piece) = board.piece_at(&new_pos) {
                            if piece.color() != color {
                                result.push(Move::new(*position, new_pos));
                            }
                        } else {
                            result.push(Move::new(*position, new_pos));
                        }
                    }
                }
            }
        }
        result
    }

    fn bishop_pseudo_legal_moves(board: &Board, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        for &dx in &[-1, 1] {
            for &dy in &[-1, 1] {
                let mut new_pos = position.offset(dx, dy);
                while let Some(pos) = new_pos {
                    if let Some(piece) = board.piece_at(&pos) {
                        if piece.color() != color {
                            result.push(Move::new(*position, pos));
                        }
                        break;
                    }
                    result.push(Move::new(*position, pos));
                    new_pos = pos.offset(dx, dy);
                }
            }
        }
        result
    }

    fn rook_pseudo_legal_moves(board: &Board, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        for &dx in &[-1, 1] {
            let mut new_pos = position.offset(dx, 0);
            while let Some(pos) = new_pos {
                if let Some(piece) = board.piece_at(&pos) {
                    if piece.color() != color {
                        result.push(Move::new(*position, pos));
                    }
                    break;
                }
                result.push(Move::new(*position, pos));
                new_pos = pos.offset(dx, 0);
            }
        }
        for &dy in &[-1, 1] {
            let mut new_pos = position.offset(0, dy);
            while let Some(pos) = new_pos {
                if let Some(piece) = board.piece_at(&pos) {
                    if piece.color() != color {
                        result.push(Move::new(*position, pos));
                    }
                    break;
                }
                result.push(Move::new(*position, pos));
                new_pos = pos.offset(0, dy);
            }
        }
        result
    }

    fn queen_pseudo_legal_moves(board: &Board, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        result.extend(Self::bishop_pseudo_legal_moves(board, position, color));
        result.extend(Self::rook_pseudo_legal_moves(board, position, color));
        result
    }

    fn king_pseudo_legal_moves(board: &Board, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx != 0 || dy != 0 {
                    if let Some(new_pos) = position.offset(dx, dy) {
                        if let Some(piece) = board.piece_at(&new_pos) {
                            if piece.color() != color {
                                result.push(Move::new(*position, new_pos));
                            }
                        } else {
                            result.push(Move::new(*position, new_pos));
                        }
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_pseudo_legal_moves() {
        let board = Board::default();
        let white_pawn = Position::new_unchecked(3, 1);
        let black_pawn = Position::new_unchecked(3, 6);
        let white_moves = MoveGenerator::pawn_pseudo_legal_moves(&board, &white_pawn, Color::White);
        let black_moves = MoveGenerator::pawn_pseudo_legal_moves(&board, &black_pawn, Color::Black);
        assert_eq!(white_moves.len(), 2);
        assert_eq!(black_moves.len(), 2);
    }

    #[test]
    fn test_knight_pseudo_legal_moves() {
        let board = Board::default();
        let knight = Position::new_unchecked(3, 3);
        let moves = MoveGenerator::knight_pseudo_legal_moves(&board, &knight, Color::White);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_bishop_pseudo_legal_moves() {
        let board = Board::default();
        let bishop = Position::new_unchecked(3, 3);
        let moves = MoveGenerator::bishop_pseudo_legal_moves(&board, &bishop, Color::White);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn test_rook_pseudo_legal_moves() {
        let board = Board::default();
        let rook = Position::new_unchecked(3, 3);
        let moves = MoveGenerator::rook_pseudo_legal_moves(&board, &rook, Color::White);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_queen_pseudo_legal_moves() {
        let board = Board::default();
        let queen = Position::new_unchecked(3, 3);
        let moves = MoveGenerator::queen_pseudo_legal_moves(&board, &queen, Color::White);
        assert_eq!(moves.len(), 27);
    }

    #[test]
    fn test_king_pseudo_legal_moves() {
        let board = Board::default();
        let king = Position::new_unchecked(3, 3);
        let moves = MoveGenerator::king_pseudo_legal_moves(&board, &king, Color::White);
        assert_eq!(moves.len(), 8);
    }
}
