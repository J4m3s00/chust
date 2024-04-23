use std::collections::HashMap;

use anyhow::Context;
use chust::{
    color::Color,
    moves::Move,
    play_game::{PlayGame, TurnResult},
    players::player_cli::CliPlayer,
    search,
};
use dotenv::dotenv;
use reqwest::Client;
use tokio::task::spawn_blocking;

use crate::{
    incoming_events::Event,
    incoming_game_state::{FullGameEvent, GameState},
    player::LichessPlayer,
};

const BASE_URL: &str = "https://lichess.org/api";

pub struct LichessServer {
    running_games: HashMap<String, RunningGame>,
    client: Client,
    auth: String,
}

struct RunningGame {
    bot_color: Color,
    tx: std::sync::mpsc::Sender<Move>,
    listen_handle: tokio::task::JoinHandle<()>,
    playing_handle: tokio::task::JoinHandle<()>,
}

impl RunningGame {
    fn receive_lichess_move(&self, state: GameState) {
        // If the play_game is behind the server, we need to update it
        let server_moves = state
            .moves
            .split_whitespace()
            .map(|m| {
                m.parse::<Move>()
                    .expect(&format!("Failed decoding move {m}"))
            })
            .collect::<Vec<_>>();

        let move_color = if server_moves.len() % 2 == 0 {
            Color::Black
        } else {
            Color::White
        };

        /*if server_moves.len() > local_moves.len() {
            let moves_to_play = server_moves.len() - local_moves.len();
            for i in 0..moves_to_play {
                let mov = server_moves[i + local_moves.len()];
                self.game.make_move(mov);
            }
        }*/

        // First we update all the moves
        if move_color != self.bot_color {
            self.tx
                .send(server_moves.into_iter().last().expect("No move given"))
                .expect("Failed to send state");
        }
    }
}

impl LichessServer {
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let auth =
            "Bearer ".to_owned() + &std::env::var("LICHESS_TOK").expect("LICHESS_TOK not set");

        Ok(Self {
            running_games: HashMap::new(),
            client: Client::default(),
            auth,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(24);
        let (game_tx, mut game_rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(handle_events(
            self.client.clone(),
            self.auth.clone(),
            event_tx,
        ));

        loop {
            tokio::select! {
                Some(event) = event_rx.recv() => {
                    if let Err(e) = self.event_received(event, game_tx.clone()).await {
                        println!("Error processing event: {:?}", e);
                    }
                }
                Some(game_event) = game_rx.recv() => {
                    let (game_id, state) = game_event;

                    if let Some(game) = self.running_games.get_mut(&game_id) {
                        game.receive_lichess_move(state);
                    }
                }
            }
        }
    }

    async fn event_received(
        &mut self,
        event: Event,
        game_sender: tokio::sync::mpsc::Sender<(String, GameState)>,
    ) -> anyhow::Result<()> {
        match event {
            Event::Challenge { challenge } => {
                println!(
                    "Accepting challenge: {} from '{}'",
                    challenge.id, challenge.challenger.name
                );
                self.send_challenge_accepted(challenge.id).await
            }
            Event::ChallengeCanceled { challenge } => {
                anyhow::bail!("Challenge canceled from '{:?}'", challenge.challenger.name)
            }
            Event::ChallengeDeclined { challenge } => {
                anyhow::bail!("Challenge declined from '{:?}'", challenge.challenger.name)
            }
            Event::GameStart(game_start) => {
                println!(
                    "Game started agains: {:?}",
                    game_start.game.opponent.username
                );
                let game_id = game_start.game.game_id.clone();
                let running_game = start_game(
                    game_start,
                    self.client.clone(),
                    self.auth.clone(),
                    game_sender,
                )
                .await?;
                self.running_games.insert(game_id, running_game);
                Ok(())
            }
            Event::GameFinish(game_end) => {
                println!(
                    "Game finished against: {:?}",
                    game_end.game.opponent.username
                );
                if let Some(removed_game) = self.running_games.remove(&game_end.game.game_id) {
                    removed_game.listen_handle.abort();
                    removed_game.playing_handle.abort();
                    println!("Removed game from running handles");
                }
                Ok(())
            }
        }
    }

    async fn send_challenge_accepted(&self, challenge_id: String) -> anyhow::Result<()> {
        let res = self
            .client
            .post(format!("{BASE_URL}/challenge/{challenge_id}/accept"))
            .header("Authorization", self.auth.clone())
            .send()
            .await
            .context("Failed to send challenge accept request")?;

        println!(
            "{}",
            res.text().await.context("Failed to get response body")?
        );

        Ok(())
    }
}

// This will wait for any event that is received
async fn handle_events(
    client: Client,
    auth: String,
    sender: tokio::sync::mpsc::Sender<Event>,
) -> anyhow::Result<()> {
    let mut res = client
        .get(format!("{BASE_URL}/stream/event"))
        .header("Authorization", auth.clone())
        .send()
        .await
        .expect("Failed to send request");
    while let Some(chunk) = res.chunk().await.context("Failed to get chunk")? {
        let chunk = std::str::from_utf8(&chunk).context("Failed to parse chunk as utf8")?;
        if chunk.trim().is_empty() {
            continue;
        }
        let event: Event = serde_json::from_str(chunk).map_err(|e| {
            println!("Event Parse error: {:?}", e);
            println!("Chunk: {:?}", chunk);
            anyhow::Error::msg("Failed to parse event")
        })?;
        sender
            .send(event)
            .await
            .context("Sending event to be processed")?;
    }

    Ok(())
}

// This will spawn a thread to receive game infos
// Returns all the handles needed to receive game infos
async fn start_game(
    game_start: crate::incoming_events::game::GameStart,
    client: Client,
    auth: String,
    send_state: tokio::sync::mpsc::Sender<(String, GameState)>,
) -> anyhow::Result<RunningGame> {
    let game = chust::fen::Fen::parse_game(&game_start.game.fen)?;
    let game_id = game_start.game.game_id.clone();

    let bot_player_color = match game_start.game.color.as_str() {
        "white" => Color::White,
        "black" => Color::Black,
        _ => anyhow::bail!("Invalid color"),
    };
    let (move_tx, mut move_rx) = tokio::sync::mpsc::channel(100);

    let game_event_handle = tokio::spawn(async move {
        let client = client.clone();
        loop {
            tokio::select! {
                Some(mov) = move_rx.recv() => {
                    client.post(format!("{BASE_URL}/bot/game/{}/move/{}", game_id, mov))
                        .header("Authorization", auth.clone())
                        .send()
                        .await
                        .expect("Failed to send move");
                }
                Err(e) = handle_game_events(client.clone(), auth.clone(), game_id.clone(), send_state.clone()) => {
                    println!("Error handling game events: {:?}", e);
                }
                else => {
                    break;
                }
            }
        }
    });

    let (tx, rx) = std::sync::mpsc::channel();

    let play_thread = spawn_blocking(move || {
        let mut game = PlayGame::default()
            .connect_player(Box::new(search::BotBasic), bot_player_color)
            .expect_waiting()
            .connect_player(
                Box::new(LichessPlayer::new(rx)),
                bot_player_color.opposite(),
            )
            .expect_ready()
            .start(game);

        while let TurnResult::InProgress(mov, color) = game.wait_for_move() {
            println!("Made move {mov}");
            game.game().print_pieces();
            if color == bot_player_color {
                move_tx.blocking_send(mov).expect("Failed to send move");
            }
        }
    });

    let running_game = RunningGame {
        bot_color: bot_player_color,
        tx,
        listen_handle: game_event_handle,
        playing_handle: play_thread,
    };
    Ok(running_game)
}

async fn handle_game_events(
    client: Client,
    auth: String,
    game_id: String,
    tx: tokio::sync::mpsc::Sender<(String, GameState)>,
) -> anyhow::Result<()> {
    let mut res = client
        .get(format!("{BASE_URL}/bot/game/stream/{}", game_id))
        .header("Authorization", auth)
        .send()
        .await
        .context("Failed to send request")?;

    while let Some(chunk) = res.chunk().await.context("Failed to get chunk")? {
        let chunk = std::str::from_utf8(&chunk).context("Failed to parse chunk as utf8")?;
        if chunk.trim().is_empty() {
            continue;
        }
        let state =
            serde_json::from_str::<FullGameEvent>(chunk).context("Deserialize FullGameEvent")?;
        let state = match state {
            FullGameEvent::GameFull { state } => state,
            FullGameEvent::GameState(state) => state,
        };
        tx.send((game_id.clone(), state))
            .await
            .context("Failed to send state_event")?;
    }

    Ok(())
}
