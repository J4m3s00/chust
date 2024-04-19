use crate::{game::Game, moves::Move};

pub mod bot_random;
pub mod player_cli;

pub trait PlayerInterface {
    fn make_move(&self, game: &Game) -> Option<Move>;
}
