use chust::game::Game;

fn main() {
    let game = Game::default();
    game.board().print_pieces();
}
