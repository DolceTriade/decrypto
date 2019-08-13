extern crate actix;
extern crate actix_files;
extern crate actix_session;
extern crate actix_web;
extern crate actix_web_actors;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate tera;
extern crate simple_logging;
extern crate uuid;
#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_session::{CookieSession, Session};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

mod decrypto;
mod game;
mod lobby;
mod state;
mod utils;

fn p404(state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    utils::render_template(state, "404.html")
}

fn main() {
    simple_logging::log_to_stderr(log::LevelFilter::Info);
    let wordlist: Vec<String> = read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/words.txt"))
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let games: Arc<Mutex<HashMap<String, Addr<decrypto::Decrypto>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let players: Arc<Mutex<HashMap<String, state::Player>>> = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        let state = state::AppState {
            template: tera,
            wordlist: wordlist.clone(),
            games: games.clone(),
            players: players.clone(),
            arbiter: Arbiter::new(),
        };
        App::new()
            .data(state)
            // cookie session middleware
            // TODO: Use real key.
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(actix_files::Files::new("/static", "./static/").show_files_listing())
            .route("/", web::get().to(lobby::lobby))
            .service(web::resource("/lobby_ws").route(web::get().to(lobby::lobby_ws)))
            .service(
                web::resource("/game/{name}")
                    .route(web::get().to(game::game))
                    .name("game"),
            )
            .service(web::resource("/game/{name}/ws").route(web::get().to(game::game_ws)))
            .default_service(web::route().to(p404))
    })
    .bind("127.0.0.1:8080")
    .expect("Could not bind to port 8080")
    .run()
    .unwrap();
}
