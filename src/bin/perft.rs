use std::io::Write;

use anyhow::Context;
use chust::{fen::Fen, game::Game, perft::PerfTest};

fn main() -> anyhow::Result<()> {
    // Get fen from stdin
    println!("Enter fen string (empty for default):");
    let mut fen = String::new();
    std::io::stdin()
        .read_line(&mut fen)
        .context("Failed to read fen from command line")?;
    let fen = fen.trim();

    // Get depth from stdin
    print!("Enter depth:");
    std::io::stdout().flush().unwrap();
    let mut depth = String::new();
    std::io::stdin()
        .read_line(&mut depth)
        .context("Failed to read depth from command line")?;
    let depth: usize = depth.trim().parse().context("Failed to parse depth")?;

    // Make the game
    let game = if fen.is_empty() {
        Game::default()
    } else {
        Fen::parse_game(fen).context("Failed to parse game from fen!")?
    };

    println!("Running perf test with depth {}...", depth);
    game.print_pieces();

    // Running perftests
    let mut perft = PerfTest::new(game, depth);
    let own_results = perft.run_perft();
    let stockfish_results = perft.run_stockfish()?;

    own_results.show_diff(&stockfish_results);

    Ok(())
}
