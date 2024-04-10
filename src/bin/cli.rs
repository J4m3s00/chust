use anyhow::Context;
use chust::{fen::Fen, game::Game, move_generation::MoveGenerator, position::Position};

fn main() -> anyhow::Result<()> {
    let mut game = Game::default();

    game.board().print_pieces();

    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        if input.is_empty() {
            continue;
        }

        let first_whitespace = input.find(char::is_whitespace).unwrap_or(input.len());
        let cmd = input[..first_whitespace].trim();
        let rest = input[first_whitespace..].trim();

        let mut handle_command = || {
            match cmd {
                "fen" => {
                    game = Fen::parse_game(rest).context("Invalid fen string!")?;
                }
                "show" => {
                    let position = rest
                        .parse::<Position>()
                        .context("Please provide a position to show possible moves")?;

                    let generator = MoveGenerator::new(&game);
                    let legal_moves = generator.pseudo_legal_moves(&position);

                    game.board().print_custom(|p| {
                        if legal_moves.iter().any(|m| m.to == p) {
                            'X'
                        } else {
                            ' '
                        }
                    });
                }
                "move" | "m" => {
                    let from = rest
                        .get(0..2)
                        .context("Please provide a move in the format 'e2e4'")?
                        .parse::<Position>()
                        .context("No valid first position given!")?;
                    let to = rest
                        .get(2..4)
                        .context("Please provide a move in the format 'e2e4'")?
                        .parse::<Position>()
                        .context("No valid second position given!")?;

                    let generator = MoveGenerator::new(&game);
                    let legal_moves = generator.pseudo_legal_moves(&from);
                    let mov = legal_moves
                        .into_iter()
                        .find(|m| m.to == to)
                        .context("Could not find valid move")?;
                    game.make_move(mov);
                    game.board().print_pieces();
                }
                "um" => {
                    game.unmake_move();
                    game.board().print_pieces();
                }
                "print" => {
                    game.board().print_pieces();
                }
                "quit" => {
                    return Ok::<Option<()>, anyhow::Error>(Some(()));
                }
                _ => {
                    println!("Unknown command: {}", cmd);
                }
            }
            Ok(None)
        };
        match handle_command() {
            Ok(Some(())) => break,
            Ok(None) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
