use actix_files as fs;
use actix_web::{get, middleware, post, put, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use harsh::Harsh;
use rand::Rng;

use crate::api;
use crate::models;

pub struct Config {
    db: String,
    bind_port: i32,
    bind_ip: i32,
    salt: String,
}

struct AppData {
    harsh: Harsh,
    pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

#[get("/game/{slug}")]
async fn get_game(
    data: web::Data<AppData>,
    slug: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let ctx = data.into_inner();
    if let Ok(ids) = ctx.harsh.decode(slug.into_inner()) {
        if let Ok(conn) = ctx.pool.get() {
            let game_id = ids[1] as i32;
            let side = if ids[2] == 0 {
                "white".to_string()
            } else {
                "black".to_string()
            };
            let game_obj = web::block(move || models::Game::find(&conn, game_id))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;
            let response = api::rest::GetGameResponse {
                created: game_obj.created,
                white: game_obj.white,
                black: game_obj.black,
                side: side,
                moves: game_obj.moves,
            };
            return Ok(HttpResponse::Ok().json(response));
        } else {
            return Err(Error::from(HttpResponse::InternalServerError().finish()));
        }
    } else {
        return Err(Error::from(HttpResponse::InternalServerError().finish()));
    }
}

#[put("/game")]
async fn put_game(
    data: web::Data<AppData>,
    form: web::Json<api::rest::CreateGameRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = data.into_inner();

    // insert to database
    if let Ok(conn) = ctx.pool.get() {
        let id = web::block(move || {
            models::Game::create(&conn, form.white.to_owned(), form.black.to_owned())
        })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        let mut rng = rand::thread_rng();
        let response = api::rest::CreateGameResponse {
            white: format!(
                "/g/{}",
                ctx.harsh
                    .encode(&[rng.gen_range(1000, 10000), id as u64, 0])
            ),
            black: format!(
                "/g/{}",
                ctx.harsh
                    .encode(&[rng.gen_range(1000, 10000), id as u64, 1])
            ),
        };
        return Ok(HttpResponse::Ok().json(response));
    } else {
        return Err(Error::from(HttpResponse::InternalServerError().finish()));
    }
}

#[post("/game/{slug}/move")]
async fn post_move(
    data: web::Data<AppData>,
    slug: web::Path<String>,
    form: web::Json<api::rest::PlayerMoveRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = data.into_inner();

    if let Ok(conn) = ctx.pool.get() {
        let result = web::block(move || {
            if let Ok(ids) = ctx.harsh.decode(slug.into_inner()) {
                let id = ids[1] as i32;
                models::Game::turn(&conn, id, &form.san)
            } else {
                Err(diesel::result::Error::from(diesel::result::Error::NotFound))
            }
        })
        .await;
        match result {
            Ok(_) => {
                let response = api::rest::PlayerMoveResponse {
                    status: true,
                    description: "Player moved".to_string(),
                };
                return Ok(HttpResponse::Ok().json(response));
            }
            Err(_) => {
                let response = api::rest::PlayerMoveResponse {
                    status: false,
                    description: "Failed player move".to_string(),
                };
                return Ok(HttpResponse::Ok().json(response));
            }
        }
    } else {
        return Err(Error::from(HttpResponse::InternalServerError().finish()));
    }
}
pub async fn start(config: &'static Config) -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new(&config.db);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("main() - Failed to create SqliteConnection pool.");
    let harsh = Harsh::builder()
        .salt(config.salt.to_owned())
        .build()
        .unwrap();
    let server = HttpServer::new(move || {
        App::new()
            .data(AppData {
                pool: pool.clone(),
                harsh: harsh.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(put_game)
            //.service(web::resource("/ws/{poll_id}").to(websocket_handler))
            .service(fs::Files::new("/static/", "static/"))
    });
    let bind = format!("{}:{}", config.bind_ip, config.bind_port);
    server.bind(&bind)?.run().await
}
