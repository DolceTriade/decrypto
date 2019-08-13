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

fn to_context(val: serde_json::Value) -> tera::Context {
    let mut ctx = tera::Context::new();
    match val {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                ctx.insert(&k, &v);
            }
        }
        _ => {
            info!("JSON value is not an object. Cannot convert into Tera context.");
        }
    }
    return ctx;
}

pub fn render_template_with_args(
    state: web::Data<state::AppState>,
    template: &str,
    json: serde_json::Value,
) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render(template, &to_context(json))
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
