extern crate actix;
extern crate actix_files;
extern crate actix_session;
extern crate actix_web;
extern crate actix_web_actors;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate simple_logging;
extern crate uuid;
#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{App, Error, HttpServer, web};
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod decrypto;
mod game;
mod lobby;
mod state;
mod utils;

pub async fn default(
    session: Session,
    _state: web::Data<state::AppState>,
) -> Result<actix_files::NamedFile, Error> {
    if session.get::<String>("uuid")?.is_none() {
        let uuid = Uuid::new_v4();
        info!("Setting UUID = {:?}", &uuid);
        session.insert("uuid", uuid.simple().to_string())?;
    }
    actix_files::NamedFile::open("./static/index.html")
        .map_err(actix_web::error::ErrorInternalServerError)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logging::log_to_stderr(log::LevelFilter::Info);

    let mut wordlist_path = format!("{}/words.txt", env!("CARGO_MANIFEST_DIR"));
    let mut bind_addr = String::from("127.0.0.1:8080");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-w" | "--wordlist" => {
                wordlist_path = args
                    .next()
                    .expect("Missing value for --wordlist/-w argument");
            }
            "-b" | "--bind" => {
                bind_addr = args.next().expect("Missing value for --bind/-b argument");
            }
            "-h" | "--help" => {
                println!("Usage: decrypto [--wordlist <path>] [--bind <addr:port>]");
                println!("Defaults:");
                println!("  --wordlist {}/words.txt", env!("CARGO_MANIFEST_DIR"));
                println!("  --bind 127.0.0.1:8080");
                return Ok(());
            }
            _ => {
                panic!(
                    "Unknown argument: {}. Use --help for usage information.",
                    arg
                );
            }
        }
    }

    let wordlist: Vec<String> = read_to_string(&wordlist_path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let games: Arc<Mutex<HashMap<String, Addr<decrypto::Decrypto>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let players: Arc<Mutex<HashMap<String, state::Player>>> = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        let state = state::AppState {
            wordlist: wordlist.clone(),
            games: games.clone(),
            players: players.clone(),
        };
        App::new()
            .app_data(web::Data::new(state))
            // cookie session middleware
            // TODO: Use real key.
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(&[0; 64]),
            ))
            .service(actix_files::Files::new("/static", "./static/").show_files_listing())
            .service(web::resource("/lobby_ws").route(web::get().to(lobby::lobby_ws)))
            .service(
                web::resource("/game/{name}")
                    .route(web::get().to(default))
                    .name("game"),
            )
            .service(web::resource("/game/{name}/ws").route(web::get().to(game::game_ws)))
            .default_service(web::route().to(default))
    })
    .bind(&bind_addr)
    .unwrap_or_else(|_| panic!("Could not bind to {}", bind_addr))
    .run()
    .await
}
