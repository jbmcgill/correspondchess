use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum PlayerSide {
    White,
    Black,
}
impl From<u64> for PlayerSide {
    fn from(x: u64) -> PlayerSide {
        match x {
            0 => PlayerSide::White,
            1 => PlayerSide::Black,
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

    #[derive(Clone, Message, Serialize, Deserialize)]
    #[rtype(result = "()")]
    pub enum Message {
        OpponentMove(PlayerMoveMessage),
    }

    impl Message {
        pub fn json(self) -> Result<String, serde_json::error::Error> {
            serde_json::to_string(&self)
        }
    }
}
