use crate::{color::Color, piece_type::PieceType};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    kind: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece_type(&self) -> PieceType {
        self.kind
    }

    pub fn get_print_char(&self) -> char {
        match (self.piece_type(), self.color()) {
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
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_char() {
        use crate::color::Color;
        use crate::piece::Piece;
        use crate::piece_type::PieceType;

        let piece = Piece::new(PieceType::Pawn, Color::White);
        assert_eq!(piece.get_print_char(), 'P');

        let piece = Piece::new(PieceType::Pawn, Color::Black);
        assert_eq!(piece.get_print_char(), 'p');

        let piece = Piece::new(PieceType::Knight, Color::White);
        assert_eq!(piece.get_print_char(), 'N');

        let piece = Piece::new(PieceType::Knight, Color::Black);
        assert_eq!(piece.get_print_char(), 'n');

        let piece = Piece::new(PieceType::Bishop, Color::White);
        assert_eq!(piece.get_print_char(), 'B');

        let piece = Piece::new(PieceType::Bishop, Color::Black);
        assert_eq!(piece.get_print_char(), 'b');

        let piece = Piece::new(PieceType::Rook, Color::White);
        assert_eq!(piece.get_print_char(), 'R');

        let piece = Piece::new(PieceType::Rook, Color::Black);
        assert_eq!(piece.get_print_char(), 'r');

        let piece = Piece::new(PieceType::Queen, Color::White);
        assert_eq!(piece.get_print_char(), 'Q');

        let piece = Piece::new(PieceType::Queen, Color::Black);
        assert_eq!(piece.get_print_char(), 'q');

        let piece = Piece::new(PieceType::King, Color::White);
        assert_eq!(piece.get_print_char(), 'K');

        let piece = Piece::new(PieceType::King, Color::Black);
        assert_eq!(piece.get_print_char(), 'k');
    }
}
