use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum PlayerSide {
    White,
    Black,
    None,
}
impl From<u64> for PlayerSide {
    fn from(x: u64) -> PlayerSide {
        match x {
            0 => PlayerSide::White,
            1 => PlayerSide::Black,
            _ => PlayerSide::None,
        }
    }
}

impl PlayerSide {
    pub fn opponent(&self) -> PlayerSide {
        match self {
            PlayerSide::White => PlayerSide::Black,
            PlayerSide::Black => PlayerSide::White,
            PlayerSide::None => PlayerSide::None,
        }
    }
}
pub mod rest {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct CreateGameRequest {
        pub white: String,
        pub black: String,
    }

    #[derive(Serialize)]
    pub struct CreateGameResponse {
        pub white: String,
        pub black: String,
    }

    #[derive(Serialize)]
    pub struct GetGameResponse {
        pub created: i64,
        pub white: String,
        pub black: String,
        pub side: String,
        pub moves: Vec<String>,
    }

    #[derive(Deserialize)]
    pub struct PlayerMoveRequest {
        pub san: String,
    }

    #[derive(Serialize)]
    pub struct PlayerMoveResponse {
        pub status: bool,
        pub description: String,
    }
}

pub mod actor {
    use actix::prelude::*;

    #[derive(Message)]
    #[rtype(usize)]
    pub struct ConnectMessage {
        pub addr: Recipient<crate::api::ws::Message>,
    }

    #[derive(Message)]
    #[rtype(result = "()")]
    pub struct DisconnectMessage {
        pub id: usize,
    }

    #[derive(Message)]
    #[rtype(result = "()")]
    pub struct SubscribeMessage {
        /// Client id
        pub id: usize,
        pub game_id: i32,
        pub side: crate::api::PlayerSide,
    }

    #[derive(Debug, Message)]
    #[rtype(result = "()")]
    pub struct NotifyMessage {
        pub key: crate::wsserver::SubscribeKey,
        pub msg: crate::api::ws::Message,
    }
}
pub mod ws {
    use actix::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PlayerMoveMessage {
        pub san: String,
    }

    impl From<String> for PlayerMoveMessage {
        fn from(s: String) -> PlayerMoveMessage {
            PlayerMoveMessage { san: s.to_owned() }
        }
    }

    #[derive(Clone, Debug, Message, Serialize, Deserialize)]
    #[rtype(result = "()")]
    pub enum Message {
        OpponentMove(PlayerMoveMessage),
    }

    impl Message {
        pub fn json(self) -> Result<String, serde_json::error::Error> {
            serde_json::to_string(&self)
        }
    }
    impl From<PlayerMoveMessage> for Message {
        fn from(x: PlayerMoveMessage) -> Message {
            Message::OpponentMove(x)
        }
    }
}
