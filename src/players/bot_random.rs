use crate::{game::Game, move_generation::MoveGenerator, moves::Move};

use super::PlayerInterface;
use rand::seq::SliceRandom;

pub struct BotRandom;

impl PlayerInterface for BotRandom {
    fn make_move(&self, game: &Game) -> Option<Move> {
        let moves = MoveGenerator::new(&game).all_legal_moves(game.current_turn());
        let mut rng = rand::thread_rng();
        moves.choose(&mut rng).cloned()
    }
}
