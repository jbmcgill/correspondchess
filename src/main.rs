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
        db: ::std::env::var("DATABASE_URL").unwrap(),
        bind_ip: "127.0.0.1".to_string(),
        bind_port: 8080,
        salt: ::std::env::var("SALT").unwrap_or("asdf".to_string()),
    };
    crate::webapp::start(config).await
}
