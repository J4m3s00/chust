use crate::{
    game::Game,
    move_generation::MoveGenerator,
    moves::{Move, MoveType},
};

use super::PlayerInterface;

pub struct CliPlayer;

impl PlayerInterface for CliPlayer {
    fn make_move(&self, game: &Game) -> Option<Move> {
        let to_make = {
            loop {
                let mut input = String::new();
                let _ = std::io::stdin().read_line(&mut input);
                let input = input.trim();
                match input.parse::<Move>() {
                    Ok(mov) => break mov,
                    Err(e) => {
                        eprintln!("Invalid move: {}", e);
                        continue;
                    }
                }
            }
        };

        let legal_moves = MoveGenerator::new(game).all_legal_moves(game.current_turn());
        legal_moves
            .iter()
            .find(|m| {
                let promotion_type = match (&to_make.move_type, &m.move_type) {
                    (
                        MoveType::PromotionQuite(a),
                        MoveType::PromotionQuite(b) | MoveType::PromotionCapture(b, _),
                    ) => a == b,
                    _ => true,
                };
                promotion_type && m.to == to_make.to && m.from == to_make.from
            })
            .cloned()
    }
}
