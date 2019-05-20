use crate::decrypto;
use crate::state;
use crate::utils;

use actix::prelude::*;
use actix::AsyncContext;
use actix_session::{Session, UserSession};
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};

pub fn game(session: Session, state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    let mut val = serde_json::Value::default();
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
        {
            let players = state.players.lock().unwrap();
            if let Some(player) = players.get(uuid) {
                val["game"] = json!(player.game);
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
    session: Session,
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<state::AppState>,
) -> Result<HttpResponse, Error> {
    println!("Starting game ws...");
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
        println!("Found UUID: {}...", &uuid);
        let mut player_opt: Option<state::Player> = None;
        {
            let players = state.players.lock().unwrap();
            if let Some(player) = players.get(uuid) {
                println!("Found player: {}", &player.name);
                player_opt.replace(player.clone());
            } else {
                return Err(error::ErrorNotFound(
                    "Player not found. Try going to the lobby.",
                ));
            }
        }
        println!("Finding game!");
        let mut game_opt: Option<Addr<decrypto::Decrypto>> = None;
        {
            let games = state.games.lock().unwrap();
            println!("There are {} games.", games.len());
            for g in &*games {
                println!("Game: {}", &g.0);
            }
            if let Some(game) = games.get(&player_opt.as_ref().unwrap().game) {
                println!("Found game!");
                game_opt.replace(game.clone());
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
        self.player.addr.replace(ctx.address());
        self.game.do_send(decrypto::AddPlayerToGame {
            uuid: self.uuid.clone(),
            player: self.player.clone(),
        });
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
    }

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("GOT: {:?}", &msg);
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
        println!("Got JSON from {}: {}", &self.player.name, &text);
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
            },
            "join_b" => {
                self.game
                    .send(decrypto::AddPlayerToTeamB {
                        player: self.player.name.clone(),
                    })
                    .wait()
                    .map_err(|e| format!("{:?}", e))??;
            },
            "leave_team" => {
                self.game
                    .send(decrypto::LeaveTeam {
                        player: self.player.name.clone(),
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
