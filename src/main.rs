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
use actix_session::{CookieSession, Session};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use uuid::prelude::*;

mod decrypto;
mod game;
mod lobby;
mod state;
mod utils;

pub fn default(session: Session, _state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    if let Ok(Some(_uuid)) = &session.get::<String>("uuid") {
    } else {
        let uuid = Uuid::new_v4();
        info!("Setting UUID = {:?}", &uuid);
        session.set("uuid", uuid.to_simple().to_string())?;
    }
    return Ok(HttpResponse::Ok().content_type("text/html").body(r##"
    <!doctype html><html lang="en"><head><meta charset="utf-8"/><link rel="shortcut icon" href="/static/favicon.ico"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="theme-color" content="#000000"/><meta name="description" content="Web site created using create-react-app"/><link rel="apple-touch-icon" href="/static/logo192.png"/><link rel="manifest" href="/static/manifest.json"/><title>Decrypto</title></head><body><noscript>You need to enable JavaScript to run this app.</noscript><div id="root"></div><script>!function(a){function e(e){for(var r,t,n=e[0],o=e[1],u=e[2],p=0,l=[];p<n.length;p++)t=n[p],Object.prototype.hasOwnProperty.call(i,t)&&i[t]&&l.push(i[t][0]),i[t]=0;for(r in o)Object.prototype.hasOwnProperty.call(o,r)&&(a[r]=o[r]);for(s&&s(e);l.length;)l.shift()();return c.push.apply(c,u||[]),f()}function f(){for(var e,r=0;r<c.length;r++){for(var t=c[r],n=!0,o=1;o<t.length;o++){var u=t[o];0!==i[u]&&(n=!1)}n&&(c.splice(r--,1),e=p(p.s=t[0]))}return e}var t={},i={1:0},c=[];function p(e){if(t[e])return t[e].exports;var r=t[e]={i:e,l:!1,exports:{}};return a[e].call(r.exports,r,r.exports,p),r.l=!0,r.exports}p.m=a,p.c=t,p.d=function(e,r,t){p.o(e,r)||Object.defineProperty(e,r,{enumerable:!0,get:t})},p.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},p.t=function(r,e){if(1&e&&(r=p(r)),8&e)return r;if(4&e&&"object"==typeof r&&r&&r.__esModule)return r;var t=Object.create(null);if(p.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:r}),2&e&&"string"!=typeof r)for(var n in r)p.d(t,n,function(e){return r[e]}.bind(null,n));return t},p.n=function(e){var r=e&&e.__esModule?function(){return e.default}:function(){return e};return p.d(r,"a",r),r},p.o=function(e,r){return Object.prototype.hasOwnProperty.call(e,r)},p.p="/";var r=window["webpackJsonpmy-app"]=window["webpackJsonpmy-app"]||[],n=r.push.bind(r);r.push=e,r=r.slice();for(var o=0;o<r.length;o++)e(r[o]);var s=n;f()}([])</script><script src="/static/js/2.6a5f4a33.chunk.js"></script><script src="/static/js/main.d002b6b5.chunk.js"></script></body></html>
    "##));
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
        let state = state::AppState {
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
            .service(web::resource("/lobby_ws").route(web::get().to(lobby::lobby_ws)))
            .service(
                web::resource("/game/{name}")
                    .route(web::get().to(default))
                    .name("game"),
            )
            .service(web::resource("/game/{name}/ws").route(web::get().to(game::game_ws)))
            .default_service(web::route().to(default))
    })
    .bind("127.0.0.1:8080")
    .expect("Could not bind to port 8080")
    .run()
    .unwrap();
}
