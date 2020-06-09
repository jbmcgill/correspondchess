use diesel::prelude::*;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::game::dsl as game_dsl;
use crate::schema::moves::dsl as moves_dsl;
use crate::schema::{game, moves};

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

#[derive(
    Identifiable, Debug, Clone, Serialize, Deserialize, Insertable, Queryable, QueryableByName,
)]
#[table_name = "game"]
pub struct Game {
    id: i32,
    created: i64,
}
#[derive(
    Associations,
    Identifiable,
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Insertable,
    Queryable,
    QueryableByName,
)]
#[table_name = "moves"]
#[belongs_to(Game)]
struct MoveRow {
    id: i32,
    created: i64,
    game_id: i32,
    player_move: String,
}

#[derive(Debug,Serialize)]
pub struct GameResponse {
    id: i32,
    created: i64,
    moves: Vec<String>,
}

impl From<(Game, Vec<MoveRow>)> for GameResponse {
    fn from(x: (Game, Vec<MoveRow>)) -> GameResponse {
        GameResponse {
            id: x.0.id,
            created: x.0.created,
            moves: x.1.iter().map(|x| x.player_move.clone()).collect(),
        }
    }
}
/// Main interface for working the games and moves data.
impl Game {
    /// Select a game by ID.
    pub fn find(conn: &SqliteConnection, id: i32) -> Result<GameResponse, diesel::result::Error> {
        let game_obj = game_dsl::game.find(id).first::<Game>(conn)?;
        let moves_list = MoveRow::belonging_to(&game_obj).load::<MoveRow>(conn)?;
        Ok(GameResponse::from((game_obj, moves_list)))
    }

    /// Create a new game. Returns the newly created game's ID.
    pub fn create(conn: &SqliteConnection) -> Result<i32, diesel::result::Error> {
        let created_at = Utc::now().timestamp_nanos();
        diesel::insert_into(game_dsl::game)
            .values(game_dsl::created.eq(created_at))
            .execute(conn)?;
        let game_id: i32 = diesel::select(last_insert_rowid).first(conn)?;
        Ok(game_id)
    }

    /// Add a player turn move to a game.
    pub fn turn(conn: &SqliteConnection, id: i32, mv: String) -> Result<(), diesel::result::Error> {
        let created_at = Utc::now().timestamp_nanos();
        diesel::insert_into(moves_dsl::moves)
            .values((moves_dsl::created.eq(created_at), moves_dsl::game_id.eq(id), moves_dsl::player_move.eq(mv)))
            .execute(conn)?;
        Ok(())
    }
}

