#[macro_use]
extern crate diesel;

mod api;
mod chess;
mod models;
mod schema;
mod webapp;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();
    let config = crate::webapp::Config {
        db: ::std::env::var("CORRESPONDCHESS_DB").unwrap_or("correspondchess.db".to_string()),
        salt: ::std::env::var("CORRESPONDCHESS_SALT").unwrap_or("kujlturenbvjccna".to_string()),
        bind: ::std::env::var("CORRESPONDCHESS_BIND").unwrap_or("127.0.0.1:8080".to_string()),
    };
    crate::webapp::start(config).await
}
