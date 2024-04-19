use crate::{color::Color, game::Game, move_generation::MoveGenerator, players::PlayerInterface};

#[derive(Default)]
pub struct WaitingForPlayers {
    possible_white_player: Option<Box<dyn PlayerInterface>>,
    possible_black_player: Option<Box<dyn PlayerInterface>>,
}

pub struct AllConnected {
    white_player: Box<dyn PlayerInterface>,
    black_player: Box<dyn PlayerInterface>,
}

pub struct Playing {
    game: Game,

    white_player: Box<dyn PlayerInterface>,
    black_player: Box<dyn PlayerInterface>,
}

pub enum ConnectResult {
    AlreadyConnected,
    Waiting(PlayGame<WaitingForPlayers>),
    Ready(PlayGame<AllConnected>),
}

pub enum TurnResult {
    Checkmate,
    Stalemate,
    InProgress,
    PlayerNotMakingMoves,
}

impl ConnectResult {
    pub fn expect_ready(self) -> PlayGame<AllConnected> {
        match self {
            ConnectResult::Ready(game) => game,
            _ => panic!("Expected Ready"),
        }
    }

    pub fn expect_waiting(self) -> PlayGame<WaitingForPlayers> {
        match self {
            ConnectResult::Waiting(game) => game,
            _ => panic!("Expected Waiting"),
        }
    }
}

pub struct PlayGame<State> {
    inner: State,
}

impl PlayGame<WaitingForPlayers> {
    pub fn new() -> PlayGame<WaitingForPlayers> {
        PlayGame {
            inner: WaitingForPlayers::default(),
        }
    }

    pub fn connect_player(self, player: Box<dyn PlayerInterface>, color: Color) -> ConnectResult {
        if color == Color::White {
            if self.inner.possible_white_player.is_some() {
                ConnectResult::AlreadyConnected
            } else if let Some(black_player) = self.inner.possible_black_player {
                ConnectResult::Ready(PlayGame {
                    inner: AllConnected {
                        white_player: player,
                        black_player,
                    },
                })
            } else {
                ConnectResult::Waiting(PlayGame {
                    inner: WaitingForPlayers {
                        possible_white_player: Some(player),
                        possible_black_player: None,
                    },
                })
            }
        } else {
            if self.inner.possible_black_player.is_some() {
                ConnectResult::AlreadyConnected
            } else if let Some(white_player) = self.inner.possible_white_player {
                ConnectResult::Ready(PlayGame {
                    inner: AllConnected {
                        white_player,
                        black_player: player,
                    },
                })
            } else {
                ConnectResult::Waiting(PlayGame {
                    inner: WaitingForPlayers {
                        possible_white_player: None,
                        possible_black_player: Some(player),
                    },
                })
            }
        }
    }
}

impl PlayGame<AllConnected> {
    pub fn start(self, game: Game) -> PlayGame<Playing> {
        PlayGame {
            inner: Playing {
                game,
                white_player: self.inner.white_player,
                black_player: self.inner.black_player,
            },
        }
    }
}

impl PlayGame<Playing> {
    pub fn wait_for_move(&mut self) -> TurnResult {
        let game = &self.inner.game;
        let player = match game.current_turn() {
            Color::White => &self.inner.white_player,
            Color::Black => &self.inner.black_player,
        };

        if MoveGenerator::new(game)
            .all_legal_moves(game.current_turn())
            .is_empty()
        {
            println!("Checkmate!");
            return TurnResult::Checkmate;
        }

        let mut try_counter = 10;
        loop {
            if let Some(mv) = player.make_move(game) {
                self.inner
                    .game
                    .make_move(mv)
                    .expect(&format!("Failed to make move"));
                break TurnResult::InProgress;
            }
            try_counter -= 1;
            if try_counter == 0 {
                println!("Player failed to make a move");
                break TurnResult::PlayerNotMakingMoves;
            }
        }
    }

    pub fn game(&self) -> &Game {
        &self.inner.game
    }
}
