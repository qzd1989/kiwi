use super::message::{error_message, ok_message, probe_request, probe_response};
use crate::commands::websocket;
use anyhow::{anyhow, Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::OnceLock;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, client_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

type Handler = fn(Value) -> Result<Value>;
static CANCEL_TOKEN: OnceLock<CancellationToken> = OnceLock::new();
const IP: &str = "127.0.0.1";

pub fn shutdown() {
    if let Some(token) = CANCEL_TOKEN.get() {
        token.cancel();
    }
}

pub async fn serve_in_background(
    port: u16,
    serve_failed_handler: Box<dyn Fn(Error) + 'static + Send>,
) -> Result<()> {
    let addr = format!("{}:{}", IP, &port);
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(error) => {
            if is_alive(port).await.unwrap_or(false) {
                return Ok(());
            }
            return Err(anyhow!(
                "Failed to bind and probe WebSocket service on port {}: {}",
                port,
                error
            ));
        }
    };
    let token = CancellationToken::new();
    let _ = CANCEL_TOKEN.set(token.clone());

    tokio::spawn(async move {
        if let Err(error) = serve(listener, token).await {
            serve_failed_handler(error);
        }
    });

    Ok(())
}

async fn serve(listener: TcpListener, token: CancellationToken) -> Result<()> {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                break;
            }
            Ok((stream, _)) = listener.accept() => {
                tokio::spawn(accept_connection(stream));
            }
        }
    }
    Ok(())
}

pub async fn is_alive(port: u16) -> Result<bool> {
    let addr = format!("{}:{}", IP, port);
    let url = format!("ws://{}", addr);
    let stream = match TcpStream::connect(addr).await {
        Ok(stream) => stream,
        Err(_) => return Ok(false),
    };
    let (ws_stream, _) = match client_async(&url, stream).await {
        Ok(result) => result,
        Err(_) => return Ok(false),
    };
    let (mut write, mut read) = ws_stream.split();

    if write.send(probe_request()).await.is_err() {
        return Ok(false);
    }

    if let Some(Ok(Message::Text(reply))) = read.next().await {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&reply) {
            if json
                .get("data")
                .and_then(|r| r.get("health"))
                .and_then(|s| s.as_str())
                == Some("ok")
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn get_method_handler(method: &str) -> Option<Handler> {
    use websocket::{frame, input};
    match method {
        "health_check" => Some(|_| Ok(probe_response())),
        "find_image" => Some(frame::find_image),
        "find_images" => Some(frame::find_images),
        "find_relative_colors" => Some(frame::find_relative_colors),
        "find_colors" => Some(frame::find_colors),
        "recognize_text" => Some(frame::recognize_text),
        "save_frame" => Some(frame::save),
        "click_left" => Some(input::click_left),
        "click_right" => Some(input::click_right),
        "press_left" => Some(input::press_left),
        "press_right" => Some(input::press_right),
        "release_left" => Some(input::release_left),
        "release_right" => Some(input::release_right),
        "move_absolute" => Some(input::move_absolute),
        "move_relative" => Some(input::move_relative),
        "get_mouse_location" => Some(input::get_mouse_location),
        "scroll_vertical" => Some(input::scroll_vertical),
        "scroll_horizontal" => Some(input::scroll_horizontal),
        "press_key" => Some(input::press_key),
        "release_key" => Some(input::release_key),
        "click_key" => Some(input::click_key),
        "input_text" => Some(input::input_text),
        _ => None,
    }
}

async fn accept_connection(stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(text) => {
                let value: Value = match serde_json::from_str(&text) {
                    Ok(v) => v,
                    Err(e) => {
                        write
                            .send(error_message(format!("Invalid JSON: {}", e)))
                            .await?;
                        continue;
                    }
                };

                let method = match value.get("method").and_then(|m| m.as_str()) {
                    Some(m) => m,
                    None => {
                        write.send(error_message("Missing 'method' field")).await?;
                        continue;
                    }
                };

                let args = value.get("args").cloned().unwrap_or(Value::Null);

                match get_method_handler(method) {
                    Some(handler) => match handler(args) {
                        Ok(result) => write.send(ok_message(result)).await?,
                        Err(e) => write.send(error_message(e.to_string())).await?,
                    },
                    None => {
                        write
                            .send(error_message(format!("Unsupported method '{}'", method)))
                            .await?;
                    }
                }
            }
            Message::Close(_) => break,
            _ => {
                write
                    .send(Message::Text("unsupported message type".into()))
                    .await?;
                continue;
            }
        }
    }

    Ok(())
}
