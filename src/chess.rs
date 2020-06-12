use shakmaty::san::San;
use shakmaty::{Chess, Position};

use crate::models::GameResponse;

#[derive(Debug)]
pub enum Error {
    InvalidMove(String),
    InvalidGameHistoryNotation(String),
    InvalidGameHistoryMove(String),
    InvalidMoveNotation(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidMove(e) => e.to_string(),
                Error::InvalidGameHistoryNotation(e) => e.to_string(),
                Error::InvalidGameHistoryMove(e) => e.to_string(),
                Error::InvalidMoveNotation(e) => e.to_string(),
            }
        )
    }
}
#[derive(Debug)]
pub enum MoveResult {
    Checkmate,
    Stalemate,
    Legal,
}
pub fn validate(g: &GameResponse, mv: &String) -> Result<MoveResult, Error> {
    let mut pos = Chess::default();

    // play through game history to set up board
    for x in g.moves.iter() {
        match San::from_ascii(x.as_bytes()) {
            Ok(san) => match san.to_move(&pos) {
                Ok(m) => match pos.play(&m) {
                    Ok(p) => pos = p,
                    Err(e) => return Err(Error::InvalidGameHistoryMove(e.to_string())),
                },
                Err(e) => return Err(Error::InvalidMoveNotation(e.to_string())),
            },
            Err(e) => {
                return Err(Error::InvalidGameHistoryNotation(format!(
                    "item: {} error: {:?}",
                    x,
                    e.to_string()
                )))
            }
        }
    }

    // try to play current move
    match San::from_ascii(mv.as_bytes()) {
        Err(e) => return Err(Error::InvalidMoveNotation(e.to_string())),
        Ok(san) => match san.to_move(&pos) {
            Err(e) => return Err(Error::InvalidMoveNotation(e.to_string())),
            Ok(m) => match pos.play(&m) {
                Ok(p) => pos = p,
                Err(e) => return Err(Error::InvalidMove(e.to_string())),
            },
        },
    }

    // check to see if move caused a win or draw
    let result = if pos.is_checkmate() {
        MoveResult::Checkmate
    } else if pos.is_stalemate() {
        MoveResult::Stalemate
    } else {
        MoveResult::Legal
    };
    Ok(result)
}
