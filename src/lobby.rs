use crate::decrypto;
use crate::state;
use crate::utils;

use actix::*;
use actix_session::{Session, UserSession};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::prelude::*;

pub fn lobby(session: Session, state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    if let Ok(Some(uuid)) = &session.get::<String>("uuid") {
        println!("UUID = {:?}", &uuid);
    } else {
        let uuid = Uuid::new_v4();
        println!("Setting UUID = {:?}", &uuid);
        session.set("uuid", uuid.to_simple().to_string())?;
    }
    utils::render_template(state, "index.html")
}

pub fn lobby_ws(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<state::AppState>,
) -> Result<HttpResponse, Error> {
    ws::start(
        Ws {
            req: req.clone(),
            state: state,
        },
        &req,
        stream,
    )
}

const MAX_NAME_LEN: usize = 15;

pub struct Ws {
    req: HttpRequest,
    state: web::Data<state::AppState>,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
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
        let cmd = &value["command"];
        if cmd.is_null() || !cmd.is_string() {
            return Err(format!("Missing or invalid command: {}", text));
        }
        match cmd.as_str().unwrap() {
            "join_or_create_game" => {
                let name = validate_and_get_name(&value["name"])?;
                let room = validate_and_get_name(&value["room"])?;
                let game_url = self
                    .req
                    .url_for("game", &[room])
                    .map_err(|e| format!("Error generating URL: {:?}", &e))?;
                self.req
                    .get_session()
                    .set("name", &name)
                    .map_err(|e| format!("Error setting name in session: {:?}", e))?;
                let mut games = self.state.games.lock().unwrap();
                if games.contains_key(&name) {
                    return Ok(
                        json!({"command": "join_game", "game": game_url.as_str()}).to_string()
                    );
                }
                games.insert(
                    name.to_string(),
                    decrypto::Decrypto::new(&self.state.wordlist),
                );
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
