use serde::Serialize;

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

pub fn json_string(json: &serde_json::Value) -> Result<String, String> {
    if !json.is_string() {
        return Err(format!("{:?} is not a string!", json));
    }
    return Ok(json.as_str().unwrap().to_string());
}
