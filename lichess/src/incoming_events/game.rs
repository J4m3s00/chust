use serde::{Deserialize, Serialize};
// Generate the struct for the json example
// Add Serialize and Deserialize derifes

/*
{
  "type": "gameFinish",
  "game": {
    "gameId": "rCRw1AuO",
    "fullId": "rCRw1AuOvonq",
    "color": "black",
    "fen": "r1bqkbnr/pppp2pp/2n1pp2/8/8/3PP3/PPPB1PPP/RN1QKBNR w KQkq - 2 4",
    "hasMoved": true,
    "isMyTurn": false,
    "lastMove": "b8c6",
    "opponent": {
      "id": "philippe",
      "username": "Philippe",
      "rating": 1790,
      "ratingDiff": -12
    },
    "perf": "correspondence",
    "rated": true,
    "secondsLeft": 1209600,
    "source": "friend",
    "status": {
      "id": 31,
      "name": "resign"
    },
    "speed": "correspondence",
    "variant": {
      "key": "standard",
      "name": "Standard"
    },
    "compat": {
      "bot": false,
      "board": true
    },
    "winner": "black",
    "ratingDiff": 8,
    "id": "rCRw1AuO"
  }
} */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameFinish {
    pub game: Game,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStart {
    pub game: Game,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_id: String,
    pub full_id: String,
    pub color: String,
    pub fen: String,
    pub has_moved: bool,
    pub is_my_turn: bool,
    pub last_move: String,
    pub opponent: User,
    pub perf: String,
    pub rated: bool,
    pub source: String,
    pub status: Status,
    pub speed: String,
    pub variant: Variant,
    pub compat: Compat,
    pub winner: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Compat {
    pub bot: bool,
    pub board: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub key: String,
    pub name: String,
}
