use crate::{game::Game, move_generation::MoveGenerator, moves::Move, players::PlayerInterface};

pub struct BotBasic;

impl PlayerInterface for BotBasic {
    fn make_move(&self, game: &Game) -> Option<Move> {
        let mut game = game.clone();
        best_move(&mut game)
    }
}

pub fn best_move(game: &mut Game) -> Option<Move> {
    let mut search = AlphaBetaSearch::new(game);
    search.minimax(3, -100000, 100000, true);
    search.best_move
}

struct AlphaBetaSearch<'a> {
    game: &'a mut Game,
    best_move: Option<Move>,
}

impl<'a> AlphaBetaSearch<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self {
            game,
            best_move: None,
        }
    }

    fn minimax(&mut self, depth: i32, alpha: i32, beta: i32, update_move: bool) -> i32 {
        if depth == 0 {
            return eval(self.game);
        }

        let mut alpha = alpha;

        let current_color = self.game.current_turn();
        let move_generator = MoveGenerator::new(self.game);
        let legal_moves = move_generator.all_legal_moves(current_color);

        if legal_moves.len() == 0 && self.game.is_in_check() {
            return -1000;
        }

        for mov in legal_moves {
            if let Err(_) = self.game.make_move(mov.clone()) {
                continue;
            }
            let eval = -self.minimax(depth - 1, -beta, -alpha, false);
            self.game.unmake_move();
            if eval >= beta {
                return beta;
            }
            if eval > alpha {
                if update_move {
                    self.best_move = Some(mov);
                }
                alpha = eval;
            }
        }

        return alpha;
    }
}

fn eval(game: &Game) -> i32 {
    let current_color = game.current_turn();
    let mut score = 0;
    score += game.bitboards().material(current_color)
        - game.bitboards().material(current_color.opposite());
    score
}
