use anyhow::Context;
use chust::{
    bitboards::BitBoardPrinter,
    fen::Fen,
    game::Game,
    move_generation::MoveGenerator,
    moves::{Move, MoveType},
    position::Position,
};

fn main() -> anyhow::Result<()> {
    let mut game = Game::default();

    game.print_pieces();

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
                    if rest.is_empty() {
                        println!("{}", Fen::from_game(&game));
                    } else {
                        game = Fen::parse_game(rest).context("Invalid fen string!")?;
                        game.print_pieces();
                    }
                }
                "show" => {
                    let position = rest
                        .parse::<Position>()
                        .context("Please provide a position to show possible moves")?;

                    let generator = MoveGenerator::new(&game);
                    let legal_moves = generator.legal_moves(&position);

                    game.print_custom(|p: Position, _: &Game| {
                        if legal_moves.iter().any(|m| m.to == p) {
                            'X'
                        } else {
                            ' '
                        }
                    });
                }
                "show_attack" => {
                    let position = rest
                        .parse::<Position>()
                        .context("Please provide a position to show possible moves")?;

                    let generator = MoveGenerator::new(&game);
                    let attacks = generator.possible_attacking_moves(&position);

                    game.print_custom(|p: Position, _: &Game| {
                        if attacks.iter().any(|m| m.to == p) {
                            'X'
                        } else {
                            ' '
                        }
                    });
                }
                "move" | "m" => {
                    let to_make = rest.parse::<Move>()?;

                    let generator = MoveGenerator::new(&game);
                    let legal_moves = generator.pseudo_legal_moves(&to_make.from);
                    let mov = legal_moves
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
                        .with_context(|| format!("Could not find valid move {:?}", legal_moves))?;
                    let _ = game.make_move(mov.clone());
                    game.print_pieces();
                }
                "um" => {
                    game.unmake_move();
                    game.print_pieces();
                }
                "print" => {
                    game.print_pieces();
                }
                "bitboard" => {
                    let which = BitBoardPrinter::ALL_IDENTIFIED
                        .iter()
                        .find(|(name, _)| name == &rest)
                        .with_context(|| {
                            let possible_values = BitBoardPrinter::ALL_IDENTIFIED.map(|i| i.0);
                            format!(
                                "Unknown bitboard {}. \nPossible values:\n{}",
                                rest,
                                possible_values.join("\n")
                            )
                        })?;
                    game.print_custom(which.1);
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
