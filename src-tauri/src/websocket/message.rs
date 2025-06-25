use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;

pub fn error_message<S: AsRef<str>>(message: S) -> Message {
    Message::Text(
        json!({
            "status": "error",
            "message": message.as_ref(),
            "data": null,
        })
        .to_string()
        .into(),
    )
}

pub fn ok_message(data: Value) -> Message {
    Message::Text(
        json!({
            "status": "success",
            "message":"operation successful",
            "data": data
        })
        .to_string()
        .into(),
    )
}

pub fn probe_request() -> Message {
    Message::Text(
        json!({
            "method": "health_check",
        })
        .to_string()
        .into(),
    )
}

pub fn probe_response() -> Value {
    json!({"health":"ok"})
}
