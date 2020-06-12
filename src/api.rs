
pub mod rest {
    use serde::{Serialize,Deserialize};

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
    pub struct PlayerMoveRequest{
        pub san: String,
    }

    #[derive(Serialize)]
    pub struct PlayerMoveResponse{
        pub status: bool,
        pub description: String,
    }
}