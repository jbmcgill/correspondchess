use actix::prelude::*;
use actix_files as fs;
use actix_web::{
    get, middleware, post, put, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use harsh::Harsh;
use rand::Rng;
use std::time::Instant;

use crate::api;
use crate::models;
use crate::wsserver;
use crate::wssession;

pub struct Config {
    pub db: String,
    pub bind: String,
    pub salt: String,
}

struct AppData {
    harsh: Harsh,
    pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
    notify: Addr<wsserver::NotifyServer>,
}

/// Entry point for our websocket route
#[get("/ws/{game_slug}")]
async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppData>,
    slug: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let ctx = data.into_inner();
    println!("webapp::websocket_handler()");
    if let Ok(ids) = ctx.harsh.decode(slug.into_inner()) {
        let game_id = ids[1] as i32;
        let side = api::PlayerSide::from(ids[2]);
        if let Ok(conn) = ctx.pool.get() {
            let game = web::block(move || models::Game::find(&conn, game_id)).await?;
            let handle = match side {
                api::PlayerSide::Black => game.black,
                api::PlayerSide::White => game.white,
                api::PlayerSide::None => {
                    return Err(Error::from(HttpResponse::InternalServerError().finish()))
                }
            };
            ws::start(
                wssession::WsSession {
                    id: 0,
                    hb: Instant::now(),
                    handle: handle,
                    subscription: wsserver::SubscribeKey { game_id, side },
                    addr: ctx.notify.clone(),
                },
                &req,
                stream,
            )
        } else {
            Err(Error::from(HttpResponse::InternalServerError().finish()))
        }
    } else {
        Err(Error::from(HttpResponse::InternalServerError().finish()))
    }
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
        let game_id: i32;
        let side: api::PlayerSide;
        let san: String = form.san.clone();
        if let Ok(ids) = ctx.harsh.decode(slug.into_inner()) {
            game_id = ids[1] as i32;
            side = api::PlayerSide::from(ids[2]);
        } else {
            return Err(Error::from(HttpResponse::InternalServerError().finish()));
        }
        let result = web::block(move || {
            models::Game::turn(&conn, game_id, form.san.clone()).map_err(|e| e.to_string())
        })
        .await;
        match result {
            Ok(_) => {
                let opponent = side.opponent();
                let key = wsserver::SubscribeKey {
                    game_id,
                    side: opponent,
                };
                let move_msg = api::ws::PlayerMoveMessage { san: san };
                let msg = api::actor::NotifyMessage {
                    key,
                    msg: api::ws::Message::from(move_msg),
                };
                println!("sending msg to notify actor {:?}", msg);
                let _ = ctx.notify.do_send(msg);

                let response = api::rest::PlayerMoveResponse {
                    status: true,
                    description: "Player moved".to_string(),
                };
                return Ok(HttpResponse::Ok().json(response));
            }
            Err(e) => {
                let response = api::rest::PlayerMoveResponse {
                    status: false,
                    description: e.to_string(),
                };
                return Ok(HttpResponse::Ok().json(response));
            }
        }
    } else {
        return Err(Error::from(HttpResponse::InternalServerError().finish()));
    }
}
pub async fn start(config: Config) -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new(&config.db);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("main() - Failed to create SqliteConnection pool.");
    let harsh = Harsh::builder()
        .salt(config.salt.to_owned())
        .build()
        .unwrap();
    let notify_addr = wsserver::NotifyServer::setup();
    let server = HttpServer::new(move || {
        App::new()
            .data(AppData {
                pool: pool.clone(),
                harsh: harsh.clone(),
                notify: notify_addr.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(put_game)
            .service(get_game)
            .service(post_move)
            .service(websocket_handler)
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    });
    server.bind(&config.bind)?.run().await
}
