#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod bitboards;
pub mod board;
pub mod color;
pub mod fen;
pub mod game;
pub mod move_generation;
pub mod moves;
pub mod perft;
pub mod piece;
pub mod piece_type;
pub mod play_game;
pub mod players;
pub mod position;
pub mod print_board;
pub mod scoped_timer;
pub mod search;
