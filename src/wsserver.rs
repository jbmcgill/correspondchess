//! `NotifyServer` is an actor. It maintains list of connection client session.
//! It rebroadcasts Notify messages to connected and subscribed clients.

use crate::api;
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq)]
pub struct SubscribeKey {
    pub game_id: i32,
    pub side: api::PlayerSide,
}

/// `NotifyServer` manages game subscriptions and is responsible for coordinating subscription
/// session.
pub struct NotifyServer {
    sessions: HashMap<usize, Recipient<api::ws::Message>>,
    games: HashMap<SubscribeKey, HashSet<usize>>,
    rng: ThreadRng,
}

impl Default for NotifyServer {
    fn default() -> NotifyServer {
        let games = HashMap::new();

        NotifyServer {
            sessions: HashMap::new(),
            games,
            rng: rand::thread_rng(),
        }
    }
}

impl NotifyServer {
    pub fn setup() -> Addr<NotifyServer> {
        actix::SyncArbiter::start(1, move || NotifyServer::default())
    }

    /// Send message to all users subscribed to game
    fn send_message(&self, subscription: SubscribeKey, message: api::ws::Message) {
        if let Some(sessions) = self.games.get(&subscription) {
            for id in sessions {
                if let Some(addr) = self.sessions.get(id) {
                    let _ = addr.do_send(message.clone());
                }
            }
        }
    }
}

impl Actor for NotifyServer {
    type Context = SyncContext<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<api::actor::ConnectMessage> for NotifyServer {
    type Result = usize;

    fn handle(
        &mut self,
        msg: api::actor::ConnectMessage,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr.clone());

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<api::actor::DisconnectMessage> for NotifyServer {
    type Result = ();

    fn handle(&mut self, msg: api::actor::DisconnectMessage, _: &mut SyncContext<Self>) {
        println!("Someone disconnected");

        if self.sessions.remove(&msg.id).is_some() {
            for (_n, sessions) in &mut self.games {
                sessions.remove(&msg.id);
            }
        }
    }
}

/// Subscribe to poll notification
impl Handler<api::actor::SubscribeMessage> for NotifyServer {
    type Result = ();

    fn handle(&mut self, msg: api::actor::SubscribeMessage, _: &mut SyncContext<Self>) {
        for (_n, sessions) in &mut self.games {
            sessions.remove(&msg.id);
        }

        self.games
            .entry(SubscribeKey {
                game_id: msg.game_id,
                side: msg.side,
            })
            .or_insert(HashSet::new())
            .insert(msg.id);
    }
}

/// Handler for Notify message.
///
/// Send a message to a game's subscribed clients. 
impl Handler<api::actor::NotifyMessage> for NotifyServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: api::actor::NotifyMessage,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let _ = self.send_message(msg.key, msg.msg);
    }
}
