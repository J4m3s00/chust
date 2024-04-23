use serde::{Deserialize, Serialize};

use self::{
    challenge::Challenge,
    game::{GameFinish, GameStart},
};

pub mod challenge;
pub mod game;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    Challenge { challenge: Challenge },
    ChallengeCanceled { challenge: Challenge },
    ChallengeDeclined { challenge: Challenge },

    GameStart(GameStart),
    GameFinish(GameFinish),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_deserialization_game() {
        let json = r#"{
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
          }"#;
        let event: Event = serde_json::from_str(json)
            .map_err(|e| println!("Event Parse error: {:?}", e))
            .unwrap();
        match event {
            Event::GameFinish(game_finish) => {
                assert_eq!(game_finish.game.game_id, "rCRw1AuO");
                assert_eq!(game_finish.game.full_id, "rCRw1AuOvonq");
                assert_eq!(game_finish.game.color, "black");
                assert_eq!(
                    game_finish.game.fen,
                    "r1bqkbnr/pppp2pp/2n1pp2/8/8/3PP3/PPPB1PPP/RN1QKBNR w KQkq - 2 4"
                );
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_event_deserialization_challenge() {
        let json = r#"{
            "type": "challenge",
            "challenge": {
              "id": "VU0nyvsW",
              "url": "https://lichess.org/VU0nyvsW",
              "color": "random",
              "direction": "out",
              "timeControl": {
                "increment": 2,
                "limit": 300,
                "show": "5+2",
                "type": "clock"
              },
              "variant": {
                "key": "standard",
                "name": "Standard",
                "short": "Std"
              },
              "challenger": {
                "id": "thibot",
                "name": "thibot",
                "online": true,
                "provisional": false,
                "rating": 1940,
                "title": "BOT"
              },
              "destUser": {
                "id": "leelachess",
                "name": "LeelaChess",
                "online": true,
                "provisional": true,
                "rating": 2670,
                "title": "BOT"
              },
              "perf": {
                "icon": ";",
                "name": "Correspondence"
              },
              "rated": true,
              "speed": "blitz",
              "status": "created"
            }
          }"#;
        let event: Event = serde_json::from_str(json)
            .map_err(|e| println!("Event Parse error: {:?}", e))
            .unwrap();
        match event {
            Event::Challenge { challenge } => {
                assert_eq!(challenge.id, "VU0nyvsW");
                //assert_eq!(challenge.direction, "out");
                assert_eq!(challenge.status, "created");
                assert_eq!(challenge.challenger.id, "thibot");
                assert_eq!(challenge.variant.key, "standard");
                //assert_eq!(challenge.time_control.limit, 300);
                //assert_eq!(challenge.time_control.increment, 2);
            }
            _ => panic!("Wrong event type"),
        }
    }
}
