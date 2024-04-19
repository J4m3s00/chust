use std::str::FromStr;

use anyhow::Context;

use crate::{
    board::Board,
    color::Color,
    game::{CastleRights, Game},
    piece::Piece,
    piece_type::PieceType,
    position::Position,
};

pub struct Fen;

impl Fen {
    pub fn from_game(game: &Game) -> String {
        let mut fen = String::new();

        for row in (0..8).rev() {
            let mut empty = 0;
            for col in 0..8 {
                let position = Position::new(col, row).unwrap();
                let piece = game.board().piece_at(&position);
                if let Some(piece) = piece {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push(piece.get_print_char());
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if row > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');

        fen.push_str(match game.current_turn() {
            Color::White => "w",
            Color::Black => "b",
        });

        fen.push(' ');

        let white_castling = game.white_castle_rights();
        let black_castling = game.black_castle_rights();
        match (white_castling, black_castling) {
            (CastleRights::None, CastleRights::None) => fen.push('-'),
            (CastleRights::None, black) => fen.push_str(black.to_string(Color::Black)),
            (white, CastleRights::None) => fen.push_str(white.to_string(Color::White)),
            (white, black) => {
                fen.push_str(white.to_string(Color::White));
                fen.push_str(black.to_string(Color::Black));
            }
        }

        fen.push(' ');

        fen.push_str(
            game.en_passent_field()
                .map(|p| p.to_string())
                .unwrap_or("-".to_string())
                .as_str(),
        );

        fen.push(' ');

        fen.push('0');

        fen.push(' ');

        fen.push('1');

        fen
    }

    pub fn parse_game(fen: &str) -> anyhow::Result<Game> {
        let mut part_iter = fen.split_whitespace();

        let board = part_iter
            .next()
            .map(|position_part| {
                let mut board = Board::default();
                // We start at the top of the board
                let mut row = 7;
                let mut col = 0;
                for c in position_part.chars() {
                    if c.is_numeric() {
                        let skip = c.to_digit(10).unwrap() as u8;
                        col += skip;
                    } else if c == '/' {
                        row -= 1;
                        col = 0;
                    } else {
                        let piece = match c {
                            'P' => Piece::new(PieceType::Pawn, Color::White),
                            'p' => Piece::new(PieceType::Pawn, Color::Black),
                            'N' => Piece::new(PieceType::Knight, Color::White),
                            'n' => Piece::new(PieceType::Knight, Color::Black),
                            'B' => Piece::new(PieceType::Bishop, Color::White),
                            'b' => Piece::new(PieceType::Bishop, Color::Black),
                            'R' => Piece::new(PieceType::Rook, Color::White),
                            'r' => Piece::new(PieceType::Rook, Color::Black),
                            'Q' => Piece::new(PieceType::Queen, Color::White),
                            'q' => Piece::new(PieceType::Queen, Color::Black),
                            'K' => Piece::new(PieceType::King, Color::White),
                            'k' => Piece::new(PieceType::King, Color::Black),
                            c => anyhow::bail!("Failed to parse fen. Unknown piece {c}"),
                        };
                        board.place_piece(
                            piece,
                            &Position::new(col, row).context(format!(
                                "Failed parsing fen. Position is getting out of bounds ({col}, {row})"
                            ))?,
                        );
                        col += 1;
                    }
                }
                Ok(board)
            })
            .context("No positions defined in the fen")??;

        let turn_color = part_iter
            .next()
            .map(|turn| match turn {
                "w" | "W" => Ok(Color::White),
                "b" | "B" => Ok(Color::Black),
                _ => {
                    anyhow::bail!(
                        "Failed to parse fen. Turn color could not be determinated. '{turn}'"
                    )
                }
            })
            .unwrap_or(Ok(Color::White))?;

        let (castle_white, castle_black) = part_iter
            .next()
            .map(|castle_rights| {
                let mut white_castle_rights = CastleRights::None;
                let mut black_castle_rights = CastleRights::None;
                for c in castle_rights.chars() {
                    match c {
                        'K' => white_castle_rights |= CastleRights::KingSide,
                        'Q' => white_castle_rights |= CastleRights::QueenSide,
                        'k' => black_castle_rights |= CastleRights::KingSide,
                        'q' => black_castle_rights |= CastleRights::QueenSide,
                        '-' => (),
                        _ => anyhow::bail!("Failed to parse fen. Unknown castle right {c}"),
                    }
                }
                Ok((white_castle_rights, black_castle_rights))
            })
            .unwrap_or(Ok((CastleRights::None, CastleRights::None)))?;

        let en_passent_field = part_iter
            .next()
            .map(|en_passent_field| {
                if en_passent_field == "-" {
                    Ok(None)
                } else {
                    Position::from_str(en_passent_field).map(Some)
                }
            })
            .unwrap_or(Ok(None))?;

        Ok(Game::new(
            board,
            turn_color,
            castle_white,
            castle_black,
            en_passent_field,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Fen;
    use crate::board::Board;
    use crate::color::Color;
    use crate::game::{CastleRights, Game};
    use crate::piece::Piece;
    use crate::piece_type::PieceType;
    use crate::position::Position;

    // Fail cases
    #[test]
    fn test_empty_string() {
        let game = Fen::parse_game("");
        assert!(game.is_err());
    }

    #[test]
    fn test_invalid_position() {
        // Test what happens when we go out of range
        let fen = "rnbqk3r/8/8/8/8/8/8/8 w KQkq - 0 1";
        let game = Fen::parse_game(fen);
        assert!(game.is_err());
    }

    #[test]
    fn test_invalid_piece() {
        // Test what happens when we have an invalid piece
        let fen = "abc5/8/8/8/8/8/8/8 w KQkq - 0 1";
        let game = Fen::parse_game(fen);
        assert!(game.is_err());
    }

    // Success cases for the default board
    #[test]
    fn test_default_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = Fen::parse_game(fen).unwrap();

        let mut expected_board = Board::default();
        expected_board.place_piece(
            Piece::new(PieceType::Rook, Color::White),
            &Position::new(0, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Knight, Color::White),
            &Position::new(1, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Bishop, Color::White),
            &Position::new(2, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Queen, Color::White),
            &Position::new(3, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::King, Color::White),
            &Position::new(4, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Bishop, Color::White),
            &Position::new(5, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Knight, Color::White),
            &Position::new(6, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Rook, Color::White),
            &Position::new(7, 0).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(0, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(1, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(2, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(3, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(4, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(5, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(6, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::White),
            &Position::new(7, 1).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(0, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(1, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(2, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(3, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(4, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(5, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(6, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Pawn, Color::Black),
            &Position::new(7, 6).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Rook, Color::Black),
            &Position::new(0, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Knight, Color::Black),
            &Position::new(1, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Bishop, Color::Black),
            &Position::new(2, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Queen, Color::Black),
            &Position::new(3, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::King, Color::Black),
            &Position::new(4, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Bishop, Color::Black),
            &Position::new(5, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Knight, Color::Black),
            &Position::new(6, 7).unwrap(),
        );
        expected_board.place_piece(
            Piece::new(PieceType::Rook, Color::Black),
            &Position::new(7, 7).unwrap(),
        );

        let expected_game = Game::new(
            expected_board,
            Color::White,
            CastleRights::Both,
            CastleRights::Both,
            None,
        );

        assert_eq!(game, expected_game);
    }
}
