use std::sync::mpsc;

use chust::{
    move_generation::MoveGenerator,
    moves::{Move, MoveType},
    players::PlayerInterface,
};

pub struct LichessPlayer {
    move_receiver: mpsc::Receiver<Move>,
}

impl LichessPlayer {
    pub fn new(move_receiver: mpsc::Receiver<Move>) -> Self {
        Self { move_receiver }
    }
}

impl PlayerInterface for LichessPlayer {
    fn make_move(&self, game: &chust::game::Game) -> Option<chust::moves::Move> {
        let move_generator = MoveGenerator::new(game);
        let legal_moves = move_generator.all_legal_moves(game.current_turn());
        let to_make = self
            .move_receiver
            //.recv_timeout(Duration::from_millis(500))
            .recv()
            .map_err(|e| format!("Failed to receive move from lichess {e})"))
            .ok()?;

        legal_moves.into_iter().find(|m| {
            let promotion_type = match (&to_make.move_type, &m.move_type) {
                (
                    MoveType::PromotionQuite(a),
                    MoveType::PromotionQuite(b) | MoveType::PromotionCapture(b, _),
                ) => a == b,
                _ => true,
            };
            promotion_type && m.to == to_make.to && m.from == to_make.from
        })
    }
}

pub struct LichessBot<P: PlayerInterface> {
    player: P,
}

impl<P> PlayerInterface for LichessBot<P>
where
    P: PlayerInterface,
{
    fn make_move(&self, game: &chust::game::Game) -> Option<chust::moves::Move> {
        let res = self.player.make_move(game);
        if let Some(mov) = &res {
            println!("Bot move: {}", mov);
        }
        res
    }
}
