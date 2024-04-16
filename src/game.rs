use crate::{
    bitboards::GameBitBoards,
    board::Board,
    color::Color,
    fen::Fen,
    moves::{Move, MoveType},
    piece::Piece,
    piece_type::PieceType,
    position::Position,
    print_board::{BoardPrinter, DefaultBoardPrinter},
};

pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    current_turn: Color,
    board: Board,
    move_stack: Vec<Move>,
    bitboards: GameBitBoards,
}

impl Default for Game {
    fn default() -> Self {
        Fen::parse_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Failed to parse default position.")
    }
}

impl Game {
    pub fn new(board: Board, current_turn: Color) -> Self {
        let mut res = Self {
            board,
            current_turn,
            move_stack: Vec::new(),
            bitboards: GameBitBoards::default(),
        };
        res.bitboards = GameBitBoards::new(&res);
        res
    }

    /// Checks if the current moving player is in check
    /// # Returns
    /// `true` if the current moving player is in check, `false` otherwise.
    /// # Example
    pub fn is_in_check(&self) -> bool {
        let king_position = self.bitboards.king(self.current_turn);
        let opponent_color = self.current_turn.opposite();
        let opponent_moves = self.bitboards.attacks(opponent_color);
        return opponent_moves.iter().any(|pos| pos == king_position);
    }

    pub fn make_move(&mut self, mov: Move) -> anyhow::Result<()> {
        let Some(piece_to_move) = self.board.piece_at(&mov.from) else {
            anyhow::bail!("No piece to move at position {:?}", mov.from);
        };
        if piece_to_move.color() != self.current_turn {
            anyhow::bail!("It's not {:?}'s turn to move.", piece_to_move.color());
        }
        match &mov.move_type {
            MoveType::Castle => {
                let root_rank = self.current_turn.root_rank();
                if mov.from.y != root_rank {
                    panic!("Invalid castle move.");
                }
                match mov.to.file() {
                    6 => {
                        // Move king
                        self.board.make_move(&mov.from, &mov.to);
                        // Move rook
                        self.board.make_move(
                            &Position::new_unchecked(7, root_rank),
                            &Position::new_unchecked(5, root_rank),
                        );
                    }
                    2 => {
                        // Move king
                        self.board.make_move(&mov.from, &mov.to);
                        // Move rook
                        self.board.make_move(
                            &Position::new_unchecked(0, root_rank),
                            &Position::new_unchecked(3, root_rank),
                        );
                    }
                    _ => panic!("Invalid castle move."),
                }
            }
            MoveType::PromotionCapture(promotion_type, _)
            | MoveType::PromotionQuite(promotion_type) => {
                self.board.make_move(&mov.from, &mov.to);
                self.board.place_piece(
                    Piece::new(promotion_type.into(), self.current_turn),
                    &mov.to,
                );
            }
            _ => self.board.make_move(&mov.from, &mov.to),
        }
        self.move_stack.push(mov);
        self.current_turn = self.current_turn.opposite();

        self.bitboards = GameBitBoards::new(self);
        Ok(())
    }

    pub fn unmake_move(&mut self) {
        let Some(mov) = self.move_stack.pop() else {
            println!("No moves to unmake.");
            return;
        };

        match mov.move_type {
            MoveType::PromotionCapture(_, piece_type) => {
                self.board.place_piece(
                    Piece::new(PieceType::Pawn, self.current_turn.opposite()),
                    &mov.from,
                );
                self.board.remove_piece(&mov.to);

                self.board
                    .place_piece(Piece::new(piece_type, self.current_turn), &mov.to);
            }
            MoveType::PromotionQuite(_) => {
                self.board.place_piece(
                    Piece::new(PieceType::Pawn, self.current_turn.opposite()),
                    &mov.from,
                );
                self.board.remove_piece(&mov.to);
            }
            MoveType::Castle => {
                let root_rank = self.current_turn.opposite().root_rank();
                if mov.to.y != root_rank {
                    panic!("Invalid castle move.");
                }
                match mov.to {
                    Position { x: 6, .. } => {
                        // Place king back
                        self.board.make_move(&mov.to, &mov.from);
                        // Place rook back
                        self.board.make_move(
                            &Position::new_unchecked(5, root_rank),
                            &Position::new_unchecked(7, root_rank),
                        );
                    }
                    Position { x: 2, .. } => {
                        // Place king back
                        self.board.make_move(&mov.to, &mov.from);
                        // Place rook back
                        self.board.make_move(
                            &Position::new_unchecked(3, root_rank),
                            &Position::new_unchecked(0, root_rank),
                        );
                    }
                    _ => panic!("Invalid castle move."),
                }
            }
            MoveType::Capture(piece_type) => {
                self.board.make_move(&mov.to, &mov.from);
                self.board
                    .place_piece(Piece::new(piece_type, self.current_turn), &mov.to);
            }
            MoveType::Quiet => {
                self.board.make_move(&mov.to, &mov.from);
            }
            MoveType::EnPassant(_) => {
                self.board.make_move(&mov.to, &mov.from);
            }
        }
        self.current_turn = self.current_turn.opposite();

        self.bitboards = GameBitBoards::new(self);
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_turn(&self) -> Color {
        self.current_turn
    }

    pub fn bitboards(&self) -> &GameBitBoards {
        &self.bitboards
    }

    /// # Example
    /// ```
    /// use crate::chust::{game::Game, print_board::DefaultBoardPrinter, piece::Piece, position::Position};
    /// let game = Game::default();
    ///
    /// // For the default builder
    /// game.print_custom(DefaultBoardPrinter);
    ///
    /// // With closure
    /// game.print_custom(|p: Position, game: &Game| {
    ///     let piece = game.board().piece_at(&p);
    ///     piece.map(|p| 'X').unwrap_or(' ')
    /// });
    /// ```
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn print_custom(&self, printer: impl BoardPrinter) {
        println!("+---+---+---+---+---+---+---+---+");
        for i in 0..8 {
            print!("|");
            for j in 0..8 {
                let pos = Position::new_unchecked(j, 7 - i);
                print!(" {} |", printer.get_char(pos, self));
            }
            println!(" {}", 8 - i);
            println!("+---+---+---+---+---+---+---+---+");
        }
        println!("  a   b   c   d   e   f   g   h  ");
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn print_pieces(&self) {
        self.print_custom(DefaultBoardPrinter);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::moves::PromotionType;

    use super::*;

    #[test]
    fn make_moves() {
        let mut game = Game::default();
        assert_eq!(game.current_turn(), Color::White);

        // Make two moves
        // d2d4, d7d6
        game.make_move(Move::new(
            Position::new_unchecked(3, 1),
            Position::new_unchecked(3, 3),
            MoveType::EnPassant(Position::new_unchecked(3, 2)),
        ))
        .unwrap();
        assert_eq!(game.current_turn(), Color::Black);
        assert_eq!(
            game.board().piece_at(&Position::new_unchecked(3, 3)),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );
        assert_eq!(game.board().piece_at(&Position::new_unchecked(3, 2)), None);
        game.make_move(Move::new(
            Position::new_unchecked(3, 6),
            Position::new_unchecked(3, 5),
            MoveType::Quiet,
        ))
        .unwrap();
        assert_eq!(game.current_turn(), Color::White);
        assert_eq!(
            game.board().piece_at(&Position::new_unchecked(3, 5)),
            Some(&Piece::new(PieceType::Pawn, Color::Black))
        );
        assert_eq!(game.board().piece_at(&Position::new_unchecked(3, 6)), None);

        // Unmake move
        game.unmake_move();
        assert_eq!(game.current_turn(), Color::Black);
        assert_eq!(game.board().piece_at(&Position::new_unchecked(3, 5)), None);
        assert_eq!(
            game.board().piece_at(&Position::new_unchecked(3, 6)),
            Some(&Piece::new(PieceType::Pawn, Color::Black))
        );

        // Unmake second move
        game.unmake_move();
        assert_eq!(game.current_turn(), Color::White);
        assert_eq!(game.board().piece_at(&Position::new_unchecked(3, 3)), None);
        assert_eq!(
            game.board().piece_at(&Position::new_unchecked(3, 1)),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );
    }

    #[test]
    fn make_castles() {
        let mut game = Fen::parse_game("8/8/8/8/8/8/8/R3K2R w - - 0 1").unwrap();

        // Castle king side for white
        game.make_move(Move::new(
            Position::from_str("e1").unwrap(),
            Position::from_str("g1").unwrap(),
            MoveType::Castle,
        ))
        .unwrap();

        assert_eq!(
            game.board().piece_at(&Position::from_str("g1").unwrap()),
            Some(&Piece::new(PieceType::King, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("f1").unwrap()),
            Some(&Piece::new(PieceType::Rook, Color::White))
        );

        // Unmake king side castle
        game.unmake_move();
        assert_eq!(
            game.board().piece_at(&Position::from_str("e1").unwrap()),
            Some(&Piece::new(PieceType::King, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("h1").unwrap()),
            Some(&Piece::new(PieceType::Rook, Color::White))
        );

        // Make queen side castle
        game.make_move(Move::new(
            Position::from_str("e1").unwrap(),
            Position::from_str("c1").unwrap(),
            MoveType::Castle,
        ))
        .unwrap();
        assert_eq!(
            game.board().piece_at(&Position::from_str("c1").unwrap()),
            Some(&Piece::new(PieceType::King, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("d1").unwrap()),
            Some(&Piece::new(PieceType::Rook, Color::White))
        );

        // Unmake queen side castle
        game.unmake_move();
        assert_eq!(
            game.board().piece_at(&Position::from_str("e1").unwrap()),
            Some(&Piece::new(PieceType::King, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("a1").unwrap()),
            Some(&Piece::new(PieceType::Rook, Color::White))
        );
    }

    #[test]
    fn make_capture() {
        let mut game = Fen::parse_game("8/8/8/4p3/5P2/8/8/8 w - - 0 1").unwrap();

        // Make capture
        game.make_move(Move::new(
            Position::from_str("f4").unwrap(),
            Position::from_str("e5").unwrap(),
            MoveType::Capture(PieceType::Pawn),
        ))
        .unwrap();

        assert_eq!(
            game.board().piece_at(&Position::from_str("e5").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("f4").unwrap()),
            None
        );

        // Unmake move
        game.unmake_move();
        assert_eq!(
            game.board().piece_at(&Position::from_str("e5").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("f4").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );
    }

    #[test]
    fn make_promotion() {
        let mut game = Fen::parse_game("5n2/4P3/8/8/8/8/8/8 w - - 0 1").unwrap();

        // Quiet promotion
        game.make_move(Move::new(
            Position::from_str("e7").unwrap(),
            Position::from_str("e8").unwrap(),
            MoveType::PromotionQuite(PromotionType::Queen),
        ))
        .unwrap();
        assert_eq!(
            game.board().piece_at(&Position::from_str("e8").unwrap()),
            Some(&Piece::new(PieceType::Queen, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("e7").unwrap()),
            None
        );

        // Unmake quiet promotion
        game.unmake_move();
        assert_eq!(
            game.board().piece_at(&Position::from_str("e8").unwrap()),
            None
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("e7").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );

        // Make capture promotion
        game.make_move(Move::new(
            Position::from_str("e7").unwrap(),
            Position::from_str("f8").unwrap(),
            MoveType::PromotionCapture(PromotionType::Queen, PieceType::Knight),
        ))
        .unwrap();
        assert_eq!(
            game.board().piece_at(&Position::from_str("f8").unwrap()),
            Some(&Piece::new(PieceType::Queen, Color::White))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("e7").unwrap()),
            None
        );
        // Unmake capture promotion
        game.unmake_move();
        assert_eq!(
            game.board().piece_at(&Position::from_str("f8").unwrap()),
            Some(&Piece::new(PieceType::Knight, Color::Black))
        );
        assert_eq!(
            game.board().piece_at(&Position::from_str("e7").unwrap()),
            Some(&Piece::new(PieceType::Pawn, Color::White))
        );
    }
}
