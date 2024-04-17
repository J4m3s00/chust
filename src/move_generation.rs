use crate::{
    color::Color,
    game::Game,
    moves::{Move, MoveType},
    piece_type::PieceType,
    position::Position,
};

pub struct MoveGenerator<'a> {
    game: &'a Game,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self { game }
    }
}

// Legal moves
impl MoveGenerator<'_> {
    /// Returns all legal moves for a piece at the given position.
    pub fn legal_moves(&self, position: &Position) -> Vec<Move> {
        self.pseudo_legal_moves(position)
            .into_iter()
            .filter(|mov| self.is_move_legal(mov))
            .collect()
    }

    fn is_move_legal(&self, mov: &Move) -> bool {
        let board = self.game.board();
        let Some(piece_to_move) = board.piece_at(&mov.from) else {
            return false;
        };

        let to_move_color = piece_to_move.color();

        let enemy_attacks = self.game.bitboards().attacks(to_move_color.opposite());

        // Check if we are in check and need to block. Moving out should be checked be the king movement
        let blockable_checks = self.game.bitboards().blockable_checks(to_move_color);

        if enemy_attacks.contains(&self.game.bitboards().king(to_move_color)) {
            // We are currently in check. We need to block, or move the king out of the way
            match piece_to_move.kind() {
                PieceType::King => {
                    // Move out of check
                    if enemy_attacks.contains(&mov.to) {
                        return false;
                    }
                }
                _ if blockable_checks.len() == 1 => {
                    let blockable = blockable_checks[0];
                    return blockable.contains(&mov.to);
                }
                _ => {
                    // We cant block, because more than one piece is attacking or we are attacked be a knight or pawn
                    return false;
                }
            }
        }

        // Check if king is moving into check and casteling rights
        if let PieceType::King = piece_to_move.kind() {
            // Filter out when the king moves into check
            if enemy_attacks.contains(&mov.to) {
                return false;
            }

            // Check if the king is castling
            if let MoveType::Castle = mov.move_type {
                let root_rank = to_move_color.root_rank();
                let castle_dir = mov.to.file() as i8 - mov.from.file() as i8;
                let castle_dir = castle_dir / castle_dir.abs();

                let all_to_check = [
                    Position::new_unchecked(mov.from.file(), root_rank),
                    Position::new_unchecked(
                        mov.from.offset(castle_dir, 0).unwrap().file(),
                        root_rank,
                    ),
                    Position::new_unchecked(
                        mov.from.offset(castle_dir * 2, 0).unwrap().file(),
                        root_rank,
                    ),
                ];

                // Check if the kind is in check
                if all_to_check.iter().any(|pos| enemy_attacks.contains(pos)) {
                    return false;
                }
            }
        }

        // Check pins
        let pinned = self.game.bitboards().pinned(to_move_color);
        if let Some(pinned) = pinned.iter().find(|board| board.contains(&mov.from)) {
            // The piece we move is pinned
            // We can only move in the pin
            return pinned.contains(&mov.to);
        }

        true
    }
}

// Pseudo legal moves are moves that are legal in terms of the rules of chess, but may not be legal
impl MoveGenerator<'_> {
    /// Returns all possible attacking moves for a piece at the given position.
    /// This includes moves that are not legal due to the king being in check.
    /// This is useful for generating moves for the king, as it needs to know all possible
    pub fn possible_attacking_moves(&self, position: &Position) -> Vec<Move> {
        let board = self.game.board();
        let Some(piece) = board.piece_at(position) else {
            return Vec::new();
        };

        match piece.kind() {
            PieceType::Pawn => self.pawn_possible_attacking_moves(position, piece.color()),
            PieceType::Knight => self.knight_pseudo_legal_moves(position, piece.color()),
            PieceType::Bishop => self.bishop_pseudo_legal_moves(position, piece.color()),
            PieceType::Rook => self.rook_pseudo_legal_moves(position, piece.color()),
            PieceType::Queen => self.queen_pseudo_legal_moves(position, piece.color()),
            PieceType::King => self.king_pseudo_legal_moves(position, piece.color()),
        }
    }

    /// Returns all pseudo legal moves for a piece at the given position.
    /// This includes moves that are not legal due to the king being in check.
    pub fn pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        let board = self.game.board();
        let Some(piece) = board.piece_at(position) else {
            return Vec::new();
        };

        match piece.kind() {
            PieceType::Pawn => self.pawn_pseudo_legal_moves(position, piece.color()),
            PieceType::Knight => self.knight_pseudo_legal_moves(position, piece.color()),
            PieceType::Bishop => self.bishop_pseudo_legal_moves(position, piece.color()),
            PieceType::Rook => self.rook_pseudo_legal_moves(position, piece.color()),
            PieceType::Queen => self.queen_pseudo_legal_moves(position, piece.color()),
            PieceType::King => self.king_pseudo_legal_moves(position, piece.color()),
        }
    }

    fn pawn_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        let direction = color.board_direction();

        // Single step forward
        if let Some(new_pos) = position.offset(0, direction) {
            if board.piece_at(&new_pos).is_none() {
                result.push(Move::new(*position, new_pos, MoveType::Quiet));

                // Double step forward
                if position.rank() == color.pawn_rank() {
                    if let Some(new_pos) = position.offset(0, 2 * direction) {
                        if board.piece_at(&new_pos).is_none() {
                            result.push(Move::new(
                                *position,
                                new_pos,
                                MoveType::EnPassant(position.offset(0, direction).unwrap()),
                            ));
                        }
                    }
                }
            }
        }

        // Capture moves
        result.extend(
            self.pawn_possible_attacking_moves(position, color)
                .into_iter()
                .filter_map(|mov| {
                    board
                        .piece_at(&mov.to)
                        .filter(|piece| piece.color() != color)
                        .map(|piece| Move::new(*position, mov.to, MoveType::Capture(piece.kind())))
                }),
        );

        result
    }

    fn pawn_possible_attacking_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let dir = color.board_direction();
        let mut res = Vec::with_capacity(2);
        for &dx in &[-1, 1] {
            if let Some(new_pos) = position.offset(dx, dir) {
                res.push(Move::new(*position, new_pos, MoveType::Quiet));
            }
        }
        res
    }

    fn knight_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-2i8, -1, 1, 2] {
            for &dy in &[-2i8, -1, 1, 2] {
                if dx.abs() != dy.abs() {
                    if let Some(new_pos) = position.offset(dx, dy) {
                        if let Some(piece) = board.piece_at(&new_pos) {
                            if piece.color() != color {
                                result.push(Move::new(
                                    *position,
                                    new_pos,
                                    MoveType::Capture(piece.kind()),
                                ));
                            }
                        } else {
                            result.push(Move::new(*position, new_pos, MoveType::Quiet));
                        }
                    }
                }
            }
        }
        result
    }

    fn bishop_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-1, 1] {
            for &dy in &[-1, 1] {
                let mut new_pos = position.offset(dx, dy);
                while let Some(pos) = new_pos {
                    if let Some(piece) = board.piece_at(&pos) {
                        if piece.color() != color {
                            result.push(Move::new(*position, pos, MoveType::Capture(piece.kind())));
                        }
                        break;
                    }
                    result.push(Move::new(*position, pos, MoveType::Quiet));
                    new_pos = pos.offset(dx, dy);
                }
            }
        }
        result
    }

    fn rook_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-1, 1] {
            let mut new_pos = position.offset(dx, 0);
            while let Some(pos) = new_pos {
                if let Some(piece) = board.piece_at(&pos) {
                    if piece.color() != color {
                        result.push(Move::new(*position, pos, MoveType::Capture(piece.kind())));
                    }
                    break;
                }
                result.push(Move::new(*position, pos, MoveType::Quiet));
                new_pos = pos.offset(dx, 0);
            }
        }
        for &dy in &[-1, 1] {
            let mut new_pos = position.offset(0, dy);
            while let Some(pos) = new_pos {
                if let Some(piece) = board.piece_at(&pos) {
                    if piece.color() != color {
                        result.push(Move::new(*position, pos, MoveType::Capture(piece.kind())));
                    }
                    break;
                }
                result.push(Move::new(*position, pos, MoveType::Quiet));
                new_pos = pos.offset(0, dy);
            }
        }
        result
    }

    fn queen_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let mut result = Vec::new();
        result.extend(self.bishop_pseudo_legal_moves(position, color));
        result.extend(self.rook_pseudo_legal_moves(position, color));
        result
    }

    fn king_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx != 0 || dy != 0 {
                    if let Some(new_pos) = position.offset(dx, dy) {
                        if let Some(piece) = board.piece_at(&new_pos) {
                            if piece.color() != color {
                                result.push(Move::new(
                                    *position,
                                    new_pos,
                                    MoveType::Capture(piece.kind()),
                                ));
                            }
                        } else {
                            result.push(Move::new(*position, new_pos, MoveType::Quiet));
                        }
                    }
                }
            }
        }

        // Castle moves
        let root_rank = color.root_rank();
        if position.rank() == root_rank && position.file() == 4 {
            // Check if the fields next to the kind are free
            if board
                .piece_at(&Position::new_unchecked(5, root_rank))
                .is_none()
                && board
                    .piece_at(&Position::new_unchecked(6, root_rank))
                    .is_none()
            {
                // Check if the rook is on the correct field
                if let Some(piece) = board.piece_at(&Position::new_unchecked(7, root_rank)) {
                    if piece.kind() == PieceType::Rook && piece.color() == color {
                        result.push(Move::new(
                            *position,
                            Position::new_unchecked(6, root_rank),
                            MoveType::Castle,
                        ));
                    }
                }
            }
            if board
                .piece_at(&Position::new_unchecked(1, root_rank))
                .is_none()
                && board
                    .piece_at(&Position::new_unchecked(2, root_rank))
                    .is_none()
                && board
                    .piece_at(&Position::new_unchecked(3, root_rank))
                    .is_none()
            {
                // Check if the rook is on the correct field
                if let Some(piece) = board.piece_at(&Position::new_unchecked(0, root_rank)) {
                    if piece.kind() == PieceType::Rook && piece.color() == color {
                        result.push(Move::new(
                            *position,
                            Position::new_unchecked(2, root_rank),
                            MoveType::Castle,
                        ));
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
    use crate::board::Board;

    #[test]
    fn test_pawn_pseudo_legal_moves() {
        let game = Game::new(Board::default(), Color::White);
        let move_generator = MoveGenerator::new(&game);

        let white_pawn = Position::new_unchecked(3, 1);
        let black_pawn = Position::new_unchecked(3, 6);
        let white_moves = move_generator.pawn_pseudo_legal_moves(&white_pawn, Color::White);
        let black_moves = move_generator.pawn_pseudo_legal_moves(&black_pawn, Color::Black);
        assert_eq!(white_moves.len(), 2);
        assert_eq!(black_moves.len(), 2);
    }

    #[test]
    fn test_knight_pseudo_legal_moves() {
        let game = Game::new(Board::default(), Color::White);
        let move_generator = MoveGenerator::new(&game);

        let knight = Position::new_unchecked(3, 3);
        let moves = move_generator.knight_pseudo_legal_moves(&knight, Color::White);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_bishop_pseudo_legal_moves() {
        let game = Game::new(Board::default(), Color::White);
        let move_generator = MoveGenerator::new(&game);

        let bishop = Position::new_unchecked(3, 3);
        let moves = move_generator.bishop_pseudo_legal_moves(&bishop, Color::White);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn test_rook_pseudo_legal_moves() {
        let game = Game::new(Board::default(), Color::White);
        let move_generator = MoveGenerator::new(&game);

        let rook = Position::new_unchecked(3, 3);
        let moves = move_generator.rook_pseudo_legal_moves(&rook, Color::White);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_queen_pseudo_legal_moves() {
        let game = Game::new(Board::default(), Color::White);
        let move_generator = MoveGenerator::new(&game);

        let queen = Position::new_unchecked(3, 3);
        let moves = move_generator.queen_pseudo_legal_moves(&queen, Color::White);
        assert_eq!(moves.len(), 27);
    }

    #[test]
    fn test_king_pseudo_legal_moves() {
        let game = Game::new(Board::default(), Color::White);
        let move_generator = MoveGenerator::new(&game);

        let king = Position::new_unchecked(3, 3);
        let moves = move_generator.king_pseudo_legal_moves(&king, Color::White);
        assert_eq!(moves.len(), 8);
    }
}
