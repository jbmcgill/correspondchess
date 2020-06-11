#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use shakmaty::san::San;
use shakmaty::{Chess, Position};

mod chess;
mod models;
mod schema;
mod webapp;
mod api;

fn main() {
    dotenv::dotenv().ok();

    let url = ::std::env::var("DATABASE_URL").unwrap();
    let conn = SqliteConnection::establish(&url).unwrap();

    match models::Game::create(&conn, "Neil".to_string(), "Chris".to_string()) {
        Ok(id) => {
            println!("New ID is {}", id);
            for x in ["f2", "d2", "e4", "e3"].iter() {
                match models::Game::turn(&conn, id, String::from(*x)) {
                    Ok(_) => println!("--> inserted move {}", x),
                    Err(e) => println!("--> ERROR: {:#?}", e),
                }
            }
            match models::Game::find(&conn, id) {
                Ok(x) => println!("find() {:#?}", x),
                Err(e) => println!("ERROR {:#?}", e),
            }
        }
        Err(e) => println!("ERROR {:?}", e),
    }

    let mut pos = Chess::default();
    for x in ["b2b4", "b7b5", "c2c4", "bxc4"].iter() {
        let san: San = San::from_ascii(x.as_bytes()).unwrap();
        let m = san.to_move(&pos).unwrap();
        pos = pos.play(&m).unwrap();
    }
    println!("{:?}", pos);
    let g = models::GameResponse {
        id: 0,
        created: 0,
        moves: vec![
            "b2b4".to_string(),
            "b7b5".to_string(),
            "c2c4".to_string(),
            "bxc4".to_string(),
        ],
    };
    let result = chess::validate(&g, "Na3".to_string());
    println!("result: {:?}", result);
    
    let result = chess::validate(&g, "Ba3".to_string());
    println!("result: {:?}", result);
}
