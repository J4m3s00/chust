use std::array;

use crate::{
    game::Game,
    move_generation::MoveGenerator,
    moves::{Move, MoveType},
    players::PlayerInterface,
    scoped_timer::ScopedTimer,
};

const MAX_DEPTH: u32 = 4;
const MAX_MOVES: usize = 4;

pub struct BotBasic;

impl PlayerInterface for BotBasic {
    fn make_move(&self, game: &Game) -> Option<Move> {
        let mut game = game.clone();
        best_move(&mut game).map(|(mov, _)| mov)
    }
}

pub fn best_moves(game: &mut Game) -> Vec<Option<(Move, i32)>> {
    let mut search = AlphaBetaSearch::new(game);
    {
        let _t = ScopedTimer::new("search");
        search.search(MAX_DEPTH, -100000, 100000, true);
    }
    println!("Looked at {} positions", search.looked_at_positions);
    println!("Skipped {} positions", search.skipped_positions);

    search.best_moves.to_vec()
}

pub fn best_move(game: &mut Game) -> Option<(Move, i32)> {
    best_moves(game).into_iter().next().flatten()
}

struct AlphaBetaSearch<'a> {
    game: &'a mut Game,
    best_moves: [Option<(Move, i32)>; MAX_MOVES],
    looked_at_positions: u32,
    skipped_positions: u32,
}

impl<'a> AlphaBetaSearch<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self {
            game,
            best_moves: array::from_fn(|_| None),
            looked_at_positions: 0,
            skipped_positions: 0,
        }
    }

    fn move_order_score(&self, mov: &Move) -> i32 {
        let mut score = 0;
        if let Some(capture_type) = mov.move_type.capture_type() {
            score += capture_type.value();
        }
        match mov.move_type {
            MoveType::Castle => score += 100,
            MoveType::PromotionQuite(_) | MoveType::PromotionCapture(_, _) => score += 400,
            _ => {}
        }
        score
    }

    fn search(&mut self, depth: u32, alpha: i32, beta: i32, update_move: bool) -> i32 {
        if depth == 0 {
            return self.alpha_beta_captures(8, alpha, beta);
        }

        let mut alpha = alpha;

        let current_color = self.game.current_turn();
        let move_generator = MoveGenerator::new(self.game);
        let mut legal_moves = move_generator.all_legal_moves(current_color);
        legal_moves.sort_by_key(|mov| std::cmp::Reverse(self.move_order_score(mov)));

        if legal_moves.is_empty() {
            return if self.game.is_in_check() { -1000 } else { 0 };
        }

        for mov in legal_moves {
            if self.game.make_move(mov).is_err() {
                println!("Failed to make move {}", mov);
                continue;
            }
            let eval = -self.search(depth - 1, -beta, -alpha, false);
            self.game.unmake_move();
            self.looked_at_positions += 1;
            if eval >= beta {
                self.skipped_positions += 1;
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }

            if update_move {
                // Find move to insert in

                if let Some(pos) = self
                    .best_moves
                    .iter()
                    .position(|m| m.map(|(_, score)| score < eval).unwrap_or(true))
                {
                    self.best_moves.copy_within(pos..(MAX_MOVES - 1), pos + 1);
                    self.best_moves[pos] = Some((mov, eval));
                }
            }
        }

        alpha
    }

    fn alpha_beta_captures(&mut self, depth: u32, alpha: i32, beta: i32) -> i32 {
        let eval = eval(self.game);

        if depth == 0 {
            return eval;
        }

        let mut alpha = alpha;

        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }

        let current_color = self.game.current_turn();
        let move_generator = MoveGenerator::new(self.game);
        let legal_moves = move_generator
            .all_legal_moves(current_color)
            .into_iter()
            .filter(|mov| mov.move_type.is_capture())
            .collect::<Vec<_>>();

        if legal_moves.is_empty() {
            return eval;
        }

        for mov in legal_moves {
            if self.game.make_move(mov).is_err() {
                continue;
            }

            let eval = -self.alpha_beta_captures(depth - 1, -beta, -alpha);
            self.game.unmake_move();
            if eval >= beta {
                self.skipped_positions += 1;
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }
        }
        alpha
    }
}

fn eval(game: &Game) -> i32 {
    let current_color = game.current_turn();
    let mut score = 0;
    score += game.bitboards().material(current_color)
        - game.bitboards().material(current_color.opposite());
    score
}
