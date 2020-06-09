#[macro_use]
extern crate diesel;
use diesel::prelude::*;

mod models;
mod schema;

fn main() {
    dotenv::dotenv().ok();

    let url = ::std::env::var("DATABASE_URL").unwrap();
    let conn = SqliteConnection::establish(&url).unwrap();

    match models::Game::create(&conn) {
        Ok(id) => {
            println!("New ID is {}", id);
            for x in ["f2","d2","e4","e3"].iter() {
                match models::Game::turn(&conn, id, String::from(*x)){
                    Ok(_) => println!("--> inserted move {}", x),
                    Err(e) => println!("--> ERROR: {:#?}", e),
                }
            }
            match models::Game::find(&conn, id) {
                Ok(x) => println!("find() {:#?}",x),
                Err(e) => println!("ERROR {:#?}", e),
            }
        }
        Err(e) => println!("ERROR {:?}", e),
    }
}
