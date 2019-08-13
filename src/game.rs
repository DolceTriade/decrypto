use crate::decrypto;
use crate::state;
use crate::utils;

use actix::prelude::*;
use actix::AsyncContext;
use actix_session::{Session, UserSession};
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};

pub fn game(
    game: web::Path<String>,
    session: Session,
    state: web::Data<state::AppState>,
) -> Result<HttpResponse, Error> {
    info!("game: {}", *game);
    let mut val = serde_json::Value::default();
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
        {
            let players = state.players.lock().unwrap();
            if let Some(player) = players.get(uuid) {
                val["game"] = json!(*game);
            } else {
                return Err(error::ErrorNotFound(
                    "Player not found. Try going to the lobby.",
                ));
            }
        }
    }
    utils::render_template_with_args(state, "game.html", val)
}

pub fn game_ws(
    game: web::Path<String>,
    session: Session,
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<state::AppState>,
) -> Result<HttpResponse, Error> {
    info!("Starting game ws...");
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
        info!("Found UUID: {}...", &uuid);
        let mut player_opt: Option<state::Player> = None;
        {
            let players = state.players.lock().unwrap();
            if let Some(player) = players.get(uuid) {
                info!("Found player: {}", &player.name);
                player_opt.replace(player.clone());
            } else {
                return Err(error::ErrorNotFound(
                    "Player not found. Try going to the lobby.",
                ));
            }
        }
        info!("Finding game!");
        let mut game_opt: Option<Addr<decrypto::Decrypto>> = None;
        {
            let games = state.games.lock().unwrap();
            info!("There are {} games.", games.len());
            for g in &*games {
                info!("Game: {}", &g.0);
            }
            if let Some(game_addr) = games.get(&*game.to_lowercase()) {
                info!("Found game!");
                let res = game_addr.do_send(decrypto::AddPlayerToGame {
                    uuid: uuid.to_string(),
                    player: state::Player::new(&player_opt.as_ref().unwrap().name, &*game),
                });
                game_opt.replace(game_addr.clone());
            } else {
                return Err(error::ErrorNotFound("Game not found. Try going to lobby."));
            }
        }

        return ws::start(
            Ws {
                uuid: uuid.to_string(),
                player: player_opt.take().unwrap(),
                game: game_opt.take().unwrap(),
            },
            &req,
            stream,
        );
    }
    Err(error::ErrorInternalServerError(""))
}

pub struct Ws {
    uuid: String,
    player: state::Player,
    game: Addr<decrypto::Decrypto>,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("player connected: {}", &self.player.name);
        self.player.addr.replace(ctx.address().clone());
        let ret = self.game.send(decrypto::PlayerConnected {
            uuid: self.uuid.clone(),
            addr: ctx.address().clone(),
        });
        info!("player_connected: {:?}", ret.wait().unwrap());
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        info!("player disconnected: {}", &self.player.name);
        self.player.addr.take();
        let ret = self.game.send(decrypto::PlayerDisconnected {
            uuid: self.uuid.clone(),
        });
        info!("player_disconnected: {:?}", ret.wait().unwrap());
    }

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        info!("GOT: {:?}", &msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => match self.handle_text(&text, ctx) {
                Ok(out) => ctx.text(out),
                Err(err) => ctx.text(utils::send_error(&err)),
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Ws {
    fn handle_text(
        &mut self,
        text: &str,
        _ctx: &mut <Ws as Actor>::Context,
    ) -> Result<String, String> {
        let value: serde_json::Value = serde_json::from_str(text)
            .map_err(|e| format!("Error parsing JSON `{}`: {:?}", &text, &e))?;
        if !value.is_object() {
            return Err(format!("Invalid json object: {}", text));
        }
        info!("Got JSON from {}: {}", &self.player.name, &text);
        let cmd = &value["command"];
        if cmd.is_null() || !cmd.is_string() {
            return Err(format!("Missing or invalid command: {}", text));
        }
        match cmd.as_str().unwrap() {
            "join_a" => {
                self.game
                    .send(decrypto::AddPlayerToTeamA {
                        player: self.player.name.clone(),
                    })
                    .wait()
                    .map_err(|e| format!("{:?}", e))??;
            }
            "join_b" => {
                self.game
                    .send(decrypto::AddPlayerToTeamB {
                        player: self.player.name.clone(),
                    })
                    .wait()
                    .map_err(|e| format!("{:?}", e))??;
            }
            "leave_team" => {
                self.game
                    .send(decrypto::LeaveTeam {
                        player: self.player.name.clone(),
                    })
                    .wait()
                    .map_err(|e| format!("{:?}", e))??;
            }
            "start_game" => {
                self.game
                    .send(decrypto::StartGame {
                        uuid: self.uuid.clone(),
                    })
                    .wait()
                    .map_err(|e| format!("{:?}", e))??;
            }
            _ => {}
        }
        return Ok("".to_string());
    }
}

#[derive(Clone, Message)]
pub struct SendCommand {
    pub json: String,
}

impl Handler<SendCommand> for Ws {
    type Result = ();

    fn handle(&mut self, msg: SendCommand, ctx: &mut Self::Context) {
        ctx.text(msg.json);
    }
}

#[derive(Clone, Message)]
pub struct ForceDisconnect {}

impl Handler<ForceDisconnect> for Ws {
    type Result = ();

    fn handle(&mut self, msg: ForceDisconnect, ctx: &mut Self::Context) {
        ctx.stop();
    }
}
