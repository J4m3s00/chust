use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use anyhow::Context;

use crate::{fen::Fen, game::Game, move_generation::MoveGenerator};

#[derive(Default, Debug)]
pub struct PerfTestResults {
    nodes: HashMap<String, u64>,
    node_count: u64,
}

impl PerfTestResults {
    pub fn show_diff(&self, other: &PerfTestResults) {
        for (mov, nodes) in self.nodes.iter() {
            if let Some(stockfish_nodes) = other.nodes.get(mov) {
                if nodes != stockfish_nodes {
                    println!(
                        "Mismatch for move {}: {} != {}",
                        mov, nodes, stockfish_nodes
                    );
                }
            } else {
                println!("Move {} not found in stockfish results", mov);
            }
        }

        // Find what the other found and we didnt
        for (mov, _) in other.nodes.iter() {
            if self.nodes.get(mov).is_none() {
                println!("Move {mov} not found in own results");
            }
        }

        if self.node_count == other.node_count {
            println!("Results match! ({} nodes)", self.node_count);
        } else {
            println!("Sockfish ({}) Self({})", other.node_count, self.node_count);
        }
    }
}

pub struct PerfTest {
    game: Game,
    depth: usize,
}

impl PerfTest {
    pub fn new(game: Game, depth: usize) -> Self {
        Self { game, depth }
    }

    pub fn run_stockfish(&self) -> anyhow::Result<PerfTestResults> {
        let fen = Fen::from_game(&self.game);

        // Check results with stockfish
        let mut command = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("Starting stockfish")?;

        {
            let mut stdin = command.stdin.take().context("Taking stdin")?;
            if !fen.is_empty() {
                writeln!(stdin, "position fen {}", fen).context("Writing to stockfish")?;
            }
            writeln!(stdin, "go perft {}", self.depth).context("Writing to stockfish")?;
        }

        let stdout = BufReader::new(command.stdout.take().context("Taking stdout")?);
        let mut stockfish_results = PerfTestResults::default();
        stdout.lines().for_each(|line| {
            let line = line.unwrap();
            //println!("{}", line);

            if line.starts_with("Nodes searched:") {
                let nodes = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                stockfish_results.node_count = nodes;
            } else {
                if let Some((mov, nodes)) = line.split_once(':') {
                    let nodes = nodes.trim();
                    let nodes = nodes.parse::<u64>().unwrap();
                    stockfish_results.nodes.insert(mov.trim().into(), nodes);
                }
            }
        });

        Ok(stockfish_results)
    }

    pub fn run_perft(&mut self) -> PerfTestResults {
        let mut result = PerfTestResults {
            nodes: HashMap::new(),
            node_count: 0,
        };

        let move_generator = MoveGenerator::new(&self.game);
        let current_color = self.game.current_turn();
        let all_legal_moves = self
            .game
            .bitboards()
            .pieces(current_color)
            .iter()
            .flat_map(|pos| move_generator.legal_moves(&pos))
            .collect::<Vec<_>>();

        for mov in all_legal_moves {
            if let Err(e) = self.game.make_move(mov.clone()) {
                println!("Failed to make move {}: {}", mov, e);
                self.game.print_pieces();
                panic!("Failed to make move");
            }
            //self.game.print_pieces();
            //std::thread::sleep(std::time::Duration::from_millis(50));
            let nodes = self.step(1);
            result.nodes.insert(mov.to_string(), nodes);
            result.node_count += nodes;

            self.game.unmake_move();
        }

        result
    }

    fn step(&mut self, cur_depth: usize) -> u64 {
        if cur_depth >= self.depth {
            return 1;
        }
        let move_generator = MoveGenerator::new(&self.game);
        let current_color = self.game.current_turn();
        let all_legal_moves = self
            .game
            .bitboards()
            .pieces(current_color)
            .iter()
            .flat_map(|pos| move_generator.legal_moves(&pos))
            .collect::<Vec<_>>();

        let mut total_moves = 0;
        for mov in all_legal_moves {
            if let Err(e) = self.game.make_move(mov.clone()) {
                println!("Failed to make move {}: {}", mov, e);
                self.game.print_pieces();
                panic!("Failed to make move");
            }
            //self.game.print_pieces();
            //std::thread::sleep(std::time::Duration::from_millis(50));
            total_moves += self.step(cur_depth + 1);
            self.game.unmake_move();
        }
        total_moves
    }
}

impl Display for PerfTestResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Perft results: {}", self.node_count)?;
        for (i, res) in self.nodes.iter() {
            writeln!(f, "{}: {}", i, res)?;
        }
        Ok(())
    }
}
