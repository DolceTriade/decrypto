use crate::state;
use actix_web::{error, web, Error, HttpResponse};
use serde::Serialize;

pub fn render_template(
    state: web::Data<state::AppState>,
    template: &str,
) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render(template, &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize)]
struct WsError {
    command: String,
    msg: String,
}

pub fn send_error(msg: &str) -> String {
    serde_json::to_string(&WsError {
        command: "error".to_owned(),
        msg: msg.to_owned(),
    })
    .unwrap_or("".to_string())
}
