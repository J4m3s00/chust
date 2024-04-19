use crate::{
    color::Color,
    game::Game,
    moves::{Move, MoveType, PromotionType},
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
    pub fn all_legal_moves(&self, color: Color) -> Vec<Move> {
        self.game
            .bitboards()
            .pieces(color)
            .iter()
            .map(|position| self.legal_moves(&position))
            .flatten()
            .collect()
    }

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

        // Check pins
        let pinned = self.game.bitboards().pinned(to_move_color);
        if let Some(pinned) = pinned.iter().find(|board| board.contains(&mov.from)) {
            // The piece we move is pinned
            // We can only move in the pin
            return pinned.contains(&mov.to);
        }

        // Check if we are in check and need to block. Moving out should be checked be the king movement
        let blockable_checks = self.game.bitboards().blockable_checks(to_move_color);

        if enemy_attacks.contains(&self.game.bitboards().king(to_move_color)) {
            // We are currently in check. We need to block, or move the king out of the way
            match piece_to_move.piece_type() {
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
                    println!("{:?}", blockable_checks);
                    // We cant block, because more than one piece is attacking or we are attacked be a knight or pawn
                    return false;
                }
            }
        }

        // Check if king is moving into check and casteling rights
        if let PieceType::King = piece_to_move.piece_type() {
            // Filter out when the king moves into check
            if enemy_attacks.contains(&mov.to) {
                return false;
            }

            // Check if the king is castling
            if let MoveType::Castle = mov.move_type {
                let root_rank = to_move_color.root_rank();
                let castle_dir = mov.to.file() as i8 - mov.from.file() as i8;
                let castle_dir = castle_dir / castle_dir.abs();

                if castle_dir == -1 {
                    // Queen side castle
                    if !self.game.castle_rights(to_move_color).queen_side() {
                        return false;
                    }
                } else if castle_dir == 1 {
                    // King side castle
                    if !self.game.castle_rights(to_move_color).king_side() {
                        return false;
                    }
                }

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

        // Special case for en passent when pawn is pinned be rook. This will not be caught by the pinned check
        // See fen: 8/8/3p4/KPp4r/4Rp1k/8/4P1P1/8 w - c6 0 1
        if let MoveType::EnPassantCapture = mov.move_type {
            // Pawn can only be pinned by rook, thats why we just search the rooks
            // Need updated pieces bitboard

            // First we check if the king is even next to us
            let king_pos = self.game.bitboards().king(to_move_color);
            if king_pos.rank() == mov.from.rank() {
                // Get side on which the pawn is
                let pawn_dir = mov.from.rank_direction(&king_pos);
                // King os next to us
                for rook_pos in self.game.bitboards().rooks(to_move_color.opposite()).iter() {
                    if rook_pos.rank() == mov.from.rank() {
                        // Rook is on the same rank as the pawn
                        // Check if the rook is pinning the pawn
                        let rook_dir = rook_pos.rank_direction(&mov.from);
                        if rook_dir == pawn_dir {
                            // Rook is pinning the pawn
                            // Check if the rook is between the king and the pawn
                            let mut cur_pos = rook_pos;
                            let mut found_pawns = 0;
                            while cur_pos != king_pos && found_pawns <= 2 {
                                if let Some(piece) = board.piece_at(&cur_pos) {
                                    if piece.piece_type() == PieceType::Pawn {
                                        found_pawns += 1;
                                    }
                                }
                                cur_pos = cur_pos.offset(rook_dir, 0).unwrap();
                            }

                            if found_pawns == 2 {
                                return false;
                            }
                        }
                    }
                }
            }
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

        match piece.piece_type() {
            PieceType::Pawn => self.pawn_possible_attacking_moves(position, piece.color()),
            PieceType::Knight => self.knight_pseudo_legal_moves(position, piece.color(), true),
            PieceType::Bishop => self.bishop_pseudo_legal_moves(position, piece.color(), true),
            PieceType::Rook => self.rook_pseudo_legal_moves(position, piece.color(), true),
            PieceType::Queen => self.queen_pseudo_legal_moves(position, piece.color(), true),
            PieceType::King => self.king_pseudo_legal_moves(position, piece.color(), true),
        }
    }

    /// Returns all pseudo legal moves for a piece at the given position.
    /// This includes moves that are not legal due to the king being in check.
    pub fn pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        let board = self.game.board();
        let Some(piece) = board.piece_at(position) else {
            return Vec::new();
        };

        match piece.piece_type() {
            PieceType::Pawn => self.pawn_pseudo_legal_moves(position, piece.color()),
            PieceType::Knight => self.knight_pseudo_legal_moves(position, piece.color(), false),
            PieceType::Bishop => self.bishop_pseudo_legal_moves(position, piece.color(), false),
            PieceType::Rook => self.rook_pseudo_legal_moves(position, piece.color(), false),
            PieceType::Queen => self.queen_pseudo_legal_moves(position, piece.color(), false),
            PieceType::King => self.king_pseudo_legal_moves(position, piece.color(), false),
        }
    }

    fn pawn_pseudo_legal_moves(&self, position: &Position, color: Color) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        let direction = color.board_direction();
        let opposite_root = color.opposite().root_rank();

        // Single step forward
        if let Some(new_pos) = position.offset(0, direction) {
            if board.piece_at(&new_pos).is_none() {
                if new_pos.rank() == opposite_root {
                    result.push(Move::new(
                        *position,
                        new_pos,
                        MoveType::PromotionQuite(PromotionType::Queen),
                    ));
                    result.push(Move::new(
                        *position,
                        new_pos,
                        MoveType::PromotionQuite(PromotionType::Rook),
                    ));
                    result.push(Move::new(
                        *position,
                        new_pos,
                        MoveType::PromotionQuite(PromotionType::Knight),
                    ));
                    result.push(Move::new(
                        *position,
                        new_pos,
                        MoveType::PromotionQuite(PromotionType::Bishop),
                    ));
                } else {
                    result.push(Move::new(*position, new_pos, MoveType::Quiet));
                }

                // Double step forward
                if position.rank() == color.pawn_rank() {
                    if let Some(new_pos) = position.offset(0, 2 * direction) {
                        if board.piece_at(&new_pos).is_none() {
                            result.push(Move::new(
                                *position,
                                new_pos,
                                MoveType::DoublePawnPush(position.offset(0, direction).unwrap()),
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
                        .map(|piece| {
                            if mov.to.rank() == opposite_root {
                                vec![
                                    Move::new(
                                        *position,
                                        mov.to,
                                        MoveType::PromotionCapture(
                                            PromotionType::Queen,
                                            piece.piece_type(),
                                        ),
                                    ),
                                    Move::new(
                                        *position,
                                        mov.to,
                                        MoveType::PromotionCapture(
                                            PromotionType::Rook,
                                            piece.piece_type(),
                                        ),
                                    ),
                                    Move::new(
                                        *position,
                                        mov.to,
                                        MoveType::PromotionCapture(
                                            PromotionType::Knight,
                                            piece.piece_type(),
                                        ),
                                    ),
                                    Move::new(
                                        *position,
                                        mov.to,
                                        MoveType::PromotionCapture(
                                            PromotionType::Bishop,
                                            piece.piece_type(),
                                        ),
                                    ),
                                ]
                            } else {
                                vec![Move::new(
                                    *position,
                                    mov.to,
                                    MoveType::Capture(piece.piece_type()),
                                )]
                            }
                        })
                })
                .flatten(),
        );

        if let Some(en_passent) = self.game.en_passent_field() {
            if let Some(new_pos) = position.offset(-1, direction) {
                if new_pos == en_passent {
                    result.push(Move::new(*position, en_passent, MoveType::EnPassantCapture));
                }
            }
            if let Some(new_pos) = position.offset(1, direction) {
                if new_pos == en_passent {
                    result.push(Move::new(*position, en_passent, MoveType::EnPassantCapture));
                }
            }
        }

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

    fn knight_pseudo_legal_moves(
        &self,
        position: &Position,
        color: Color,
        friendly_attacks: bool,
    ) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-2i8, -1, 1, 2] {
            for &dy in &[-2i8, -1, 1, 2] {
                if dx.abs() != dy.abs() {
                    if let Some(new_pos) = position.offset(dx, dy) {
                        if let Some(piece) = board.piece_at(&new_pos) {
                            if piece.color() != color || friendly_attacks {
                                result.push(Move::new(
                                    *position,
                                    new_pos,
                                    MoveType::Capture(piece.piece_type()),
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

    fn bishop_pseudo_legal_moves(
        &self,
        position: &Position,
        color: Color,
        friendly_attacks: bool,
    ) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-1, 1] {
            for &dy in &[-1, 1] {
                let mut new_pos = position.offset(dx, dy);
                while let Some(pos) = new_pos {
                    if let Some(piece) = board.piece_at(&pos) {
                        if piece.color() != color || friendly_attacks {
                            result.push(Move::new(
                                *position,
                                pos,
                                MoveType::Capture(piece.piece_type()),
                            ));
                        }
                        if !friendly_attacks
                            || (!matches!(piece.piece_type(), PieceType::King)
                                || piece.color() == color)
                        {
                            break;
                        }
                    }
                    result.push(Move::new(*position, pos, MoveType::Quiet));
                    new_pos = pos.offset(dx, dy);
                }
            }
        }
        result
    }

    fn rook_pseudo_legal_moves(
        &self,
        position: &Position,
        color: Color,
        friendly_attacks: bool,
    ) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-1, 1] {
            let mut new_pos = position.offset(dx, 0);
            while let Some(pos) = new_pos {
                if let Some(piece) = board.piece_at(&pos) {
                    if piece.color() != color || friendly_attacks {
                        result.push(Move::new(
                            *position,
                            pos,
                            MoveType::Capture(piece.piece_type()),
                        ));
                    }
                    if !friendly_attacks
                        || (!matches!(piece.piece_type(), PieceType::King)
                            || piece.color() == color)
                    {
                        break;
                    }
                }
                result.push(Move::new(*position, pos, MoveType::Quiet));
                new_pos = pos.offset(dx, 0);
            }
        }
        for &dy in &[-1, 1] {
            let mut new_pos = position.offset(0, dy);
            while let Some(pos) = new_pos {
                if let Some(piece) = board.piece_at(&pos) {
                    if piece.color() != color || friendly_attacks {
                        result.push(Move::new(
                            *position,
                            pos,
                            MoveType::Capture(piece.piece_type()),
                        ));
                    }
                    if !friendly_attacks
                        || (!matches!(piece.piece_type(), PieceType::King)
                            || piece.color() == color)
                    {
                        break;
                    }
                }
                result.push(Move::new(*position, pos, MoveType::Quiet));
                new_pos = pos.offset(0, dy);
            }
        }
        result
    }

    fn queen_pseudo_legal_moves(
        &self,
        position: &Position,
        color: Color,
        friendly_attacks: bool,
    ) -> Vec<Move> {
        let mut result = Vec::new();
        result.extend(self.bishop_pseudo_legal_moves(position, color, friendly_attacks));
        result.extend(self.rook_pseudo_legal_moves(position, color, friendly_attacks));
        result
    }

    fn king_pseudo_legal_moves(
        &self,
        position: &Position,
        color: Color,
        frindly_attacks: bool,
    ) -> Vec<Move> {
        let board = self.game.board();
        let mut result = Vec::new();
        for &dx in &[-1, 0, 1] {
            for &dy in &[-1, 0, 1] {
                if dx != 0 || dy != 0 {
                    if let Some(new_pos) = position.offset(dx, dy) {
                        if let Some(piece) = board.piece_at(&new_pos) {
                            if piece.color() != color || frindly_attacks {
                                result.push(Move::new(
                                    *position,
                                    new_pos,
                                    MoveType::Capture(piece.piece_type()),
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
                    if piece.piece_type() == PieceType::Rook && piece.color() == color {
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
                    if piece.piece_type() == PieceType::Rook && piece.color() == color {
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
    use crate::{board::Board, fen::Fen, game::CastleRights, piece::Piece};

    #[test]
    fn test_pawn_pseudo_legal_moves() {
        let game = Game::new(
            Board::default(),
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );
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
        let knight_position = Position::new_unchecked(3, 3);
        let mut board = Board::default();
        board.place_piece(
            Piece::new(PieceType::Knight, Color::White),
            &knight_position,
        );

        let game = Game::new(
            board,
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );

        let move_generator = MoveGenerator::new(&game);
        let moves = move_generator.pseudo_legal_moves(&knight_position);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_bishop_pseudo_legal_moves() {
        let bishop_position = Position::new_unchecked(3, 3);
        let mut board = Board::default();
        board.place_piece(
            Piece::new(PieceType::Bishop, Color::White),
            &bishop_position,
        );

        let game = Game::new(
            board,
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );
        let move_generator = MoveGenerator::new(&game);

        let moves = move_generator.pseudo_legal_moves(&bishop_position);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn test_rook_pseudo_legal_moves() {
        let rook_position = Position::new_unchecked(3, 3);

        let mut board = Board::default();
        board.place_piece(Piece::new(PieceType::Rook, Color::White), &rook_position);

        let game = Game::new(
            board,
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );
        let move_generator = MoveGenerator::new(&game);

        let moves = move_generator.pseudo_legal_moves(&rook_position);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_queen_pseudo_legal_moves() {
        let queen_position = Position::new_unchecked(3, 3);

        let mut board = Board::default();
        board.place_piece(Piece::new(PieceType::Queen, Color::White), &queen_position);

        let game = Game::new(
            board,
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );
        let move_generator = MoveGenerator::new(&game);

        let moves = move_generator.pseudo_legal_moves(&queen_position);
        assert_eq!(moves.len(), 27);
    }

    #[test]
    fn test_king_pseudo_legal_moves() {
        let king_position = Position::new_unchecked(3, 3);

        let mut board = Board::default();
        board.place_piece(Piece::new(PieceType::King, Color::White), &king_position);

        let game = Game::new(
            board,
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );
        let move_generator = MoveGenerator::new(&game);

        let moves = move_generator.pseudo_legal_moves(&king_position);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn legal_move_pin() {
        let fen = "8/8/3p4/K1pP3r/4Rp1k/8/4P1P1/8 b - c6 0 1";

        test_legal_moves(fen, 0, &&Position::F4);
    }

    #[test]
    fn legal_move_dont_move_in_check() {
        let fen = "8/8/3p4/K1pP3r/4Rp1k/8/4P1P1/8 b - c6 0 1";

        test_legal_moves(fen, 3, &Position::H4);
    }

    #[test]
    fn legal_special_en_passant() {
        let fen = "8/8/3p4/K1pP3r/4Rp1k/8/4P1P1/8 b - c6 0 1";
        test_legal_moves(fen, 0, &Position::D5);
    }

    #[test]
    fn legal_promotions() {
        let fen = "2n5/3P4/8/8/8/8/8/8 w - - 0 1";
        test_legal_moves(fen, 8, &Position::D7);
    }

    #[test]
    fn legal_move_in_check() {
        let fen = "8/8/3p4/K1pP3r/4R2k/5p2/4P1P1/8 b - - 0 1";

        test_legal_moves(fen, 2, &Position::H4);
    }
    #[test]
    fn legal_casteling() {
        {
            // Normal and with through check
            let fen = "5r2/8/8/8/8/8/8/R3K2R w KQ - 0 1";

            test_legal_moves(fen, 4, &Position::E1);
        }
        {
            // No castle rights
            let fen = "8/8/8/8/8/8/8/R3K2R w - - 0 1";

            test_legal_moves(fen, 5, &Position::E1);
        }
    }
    #[test]
    fn blockable_checks() {
        {
            // Rook
            let fen = "2n1r3/3P4/8/8/8/3N4/8/4K3 b - - 0 1";

            test_legal_moves(fen, 1, &Position::D3);
        }
        {
            // Bishop
            let fen = "2n5/3P4/8/8/7b/3N4/8/4K3 b - - 0 1";
            test_legal_moves(fen, 1, &Position::D3);
        }
        {
            // Both
            let fen = "2n1r3/3P4/8/8/7b/3N4/8/4K3 b - - 0 1";
            test_legal_moves(fen, 0, &Position::D3);
        }
    }

    fn test_legal_moves(fen: &str, expected_moves: usize, piece_to_check: &Position) {
        let game = Fen::parse_game(fen).unwrap();
        let move_generator = MoveGenerator::new(&game);
        let legal_moves = move_generator.legal_moves(piece_to_check);
        assert_eq!(legal_moves.len(), expected_moves);
    }
}
