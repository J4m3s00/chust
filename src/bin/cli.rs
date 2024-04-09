#![cfg_attr(coverage, feature(coverage_attribute))]

use chust::game::Game;
#[cfg_attr(coverage, coverage(off))]
fn main() {
    let game = Game::default();
    game.board().print_pieces();
}
