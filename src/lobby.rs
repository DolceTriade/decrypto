use crate::decrypto;
use crate::state;
use crate::utils;

use actix::prelude::*;
use actix::*;
use actix_session::{Session, UserSession};
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::Arc;
use uuid::prelude::*;

pub fn lobby(session: Session, state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
    } else {
        let uuid = Uuid::new_v4();
        info!("Setting UUID = {:?}", &uuid);
        session.set("uuid", uuid.to_simple().to_string())?;
    }
    utils::render_template(state, "index.html")
}

pub fn lobby_ws(
    session: Session,
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<state::AppState>,
) -> Result<HttpResponse, Error> {
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
        return ws::start(
            Ws {
                req: req.clone(),
                state: state,
                uuid: uuid.to_string(),
            },
            &req,
            stream,
        );
    }
    Err(error::ErrorInternalServerError(""))
}

const MAX_NAME_LEN: usize = 15;

pub struct Ws {
    req: HttpRequest,
    state: web::Data<state::AppState>,
    uuid: String,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
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
        let cmd = &value["command"];
        if cmd.is_null() || !cmd.is_string() {
            return Err(format!("Missing or invalid command: {}", text));
        }
        match cmd.as_str().unwrap() {
            "join_or_create_game" => {
                let name = validate_and_get_name(&value["name"])?;
                let room = validate_and_get_name(&value["room"])?;
                let args = vec![&room];
                let game_url = self
                    .req
                    .url_for("game", &args)
                    .map_err(|e| format!("Error generating URL: {:?}", &e))?;
                {
                    let mut players = self.state.players.lock().unwrap();
                    for player in &*players {
                        if player.1.name == name && &self.uuid != player.0 {
                            return Err(format!("{} already in use!", name));
                        }
                    }
                    players.insert(self.uuid.clone(), state::Player::new(&name, &room));
                }
                let mut games = self.state.games.lock().unwrap();
                if let Some(game_addr) = games.get_mut(&room.to_lowercase()) {
                    game_addr.do_send(decrypto::AddPlayerToGame {
                        uuid: self.uuid.clone(),
                        player: state::Player::new(&name, &room),
                    });
                    return Ok(
                        json!({"command": "join_game", "game": game_url.as_str()}).to_string()
                    );
                }
                let words = self.state.wordlist.clone();
                let game_addr = self
                    .state
                    .arbiter
                    .exec(move || decrypto::Decrypto::new(&words).start())
                    .wait()
                    .unwrap();
                games.insert(room.to_lowercase(), game_addr.clone());
                game_addr.do_send(decrypto::AddPlayerToGame {
                    uuid: self.uuid.clone(),
                    player: state::Player::new(&name, &room),
                });
                return Ok(json!({"command": "join_game", "game": game_url.as_str()}).to_string());
            }
            _ => {}
        }
        return Ok("".to_string());
    }
}

fn validate_and_get_name(v: &serde_json::Value) -> Result<String, String> {
    if v.is_null() || !v.is_string() {
        return Err("Missing or invalid player name.".to_string());
    }
    let s = v.as_str().unwrap();
    if s.len() > MAX_NAME_LEN {
        return Err(format!(
            "{} name too long! Must be less than {} characters.",
            &s, &MAX_NAME_LEN
        ));
    }
    return Ok(s.to_string());
}
