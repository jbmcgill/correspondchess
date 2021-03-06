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
    white: String,
    black: String,
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
    game_id: i32,
    player_move: String,
}

#[derive(Debug, Serialize)]
pub struct GameResponse {
    pub id: i32,
    pub white: String,
    pub black: String,
    pub created: i64,
    pub moves: Vec<String>,
}

impl From<(Game, Vec<MoveRow>)> for GameResponse {
    fn from(mut x: (Game, Vec<MoveRow>)) -> GameResponse {
        x.1.sort_by(|a, b| a.id.cmp(&b.id));
        GameResponse {
            id: x.0.id,
            white: x.0.white,
            black: x.0.black,
            created: x.0.created,
            moves: x.1.iter().map(|x| x.player_move.to_owned()).collect(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
  Db(diesel::result::Error),
  Chess(crate::chess::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::Db(e) => e.to_string(),
            Error::Chess(e) => e.to_string(),
        })
    }
}
/// Main interface for working the games and moves data.
impl Game {
    /// Select a game by ID.
    pub fn find(conn: &SqliteConnection, id: i32) -> Result<GameResponse, Error> {
        let game_obj = game_dsl::game.find(id).first::<Game>(conn).map_err(|e|Error::Db(e))?;
        let moves_list = MoveRow::belonging_to(&game_obj).load::<MoveRow>(conn).map_err(|e|Error::Db(e))?;
        Ok(GameResponse::from((game_obj, moves_list)))
    }

    /// Create a new game. Returns the newly created game's ID.
    pub fn create(
        conn: &SqliteConnection,
        white: String,
        black: String,
    ) -> Result<i32, Error> {
        let created_at = Utc::now().timestamp_nanos();
        diesel::insert_into(game_dsl::game)
            .values((
                game_dsl::created.eq(created_at),
                game_dsl::white.eq(white),
                game_dsl::black.eq(black),
            ))
            .execute(conn).map_err(|e|Error::Db(e))?;
        let game_id: i32 = diesel::select(last_insert_rowid).first(conn).map_err(|e|Error::Db(e))?;
        Ok(game_id)
    }

    /// Add a player turn move to a game.
    pub fn turn(conn: &SqliteConnection, id: i32, mv: String) -> Result<(), Error> {
        let game_obj = Game::find(conn, id)?;
        match crate::chess::validate(&game_obj, &mv) {
            Ok(_) => {
                diesel::insert_into(moves_dsl::moves)
                    .values((moves_dsl::game_id.eq(id), moves_dsl::player_move.eq(mv)))
                    .execute(conn).map_err(|e|Error::Db(e))?;
                Ok(())
            }
            Err(e) => Err(Error::Chess(e)),
        }
    }
}
