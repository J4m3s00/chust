use crate::{
    board::Board,
    color::Color,
    fen::Fen,
    moves::{Move, MoveType},
    piece::Piece,
    piece_type::PieceType,
    position::Position,
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
}

impl Default for Game {
    fn default() -> Self {
        Fen::parse_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Failed to parse default position.")
    }
}

impl Game {
    pub fn new(board: Board, current_turn: Color) -> Self {
        Self {
            board,
            current_turn,
            move_stack: Vec::new(),
        }
    }

    pub fn make_move(&mut self, mov: Move) {
        let Some(piece_to_move) = self.board.piece_at(&mov.from) else {
            println!("No piece to move at position {:?}", mov.from);
            return;
        };
        if piece_to_move.color() != self.current_turn {
            println!("It's not {:?}'s turn to move.", piece_to_move.color());
            return;
        }
        match mov.move_type {
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
            _ => self.board.make_move(&mov.from, &mov.to),
        }
        self.move_stack.push(mov);
        self.current_turn = self.current_turn.opposite();
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
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_turn(&self) -> Color {
        self.current_turn
    }
}
