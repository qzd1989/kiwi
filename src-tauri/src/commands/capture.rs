use crate::capture;
use crate::common::Size;
use serde_json::json;
use tauri::ipc::Response;

#[tauri::command]
pub async fn snapshot() -> Response {
    let display = capture::engine::get_primary_display().await;
    let frame = capture::engine::snapshot(display).await;
    Response::new(frame.buffer)
}

#[tauri::command]
pub async fn display_size() -> serde_json::Value {
    let display = capture::engine::get_primary_display().await;
    let rect = display.rect();
    let size = Size::new(rect.size.width, rect.size.height);
    json!(size)
}
