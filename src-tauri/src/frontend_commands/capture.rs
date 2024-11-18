use crate::capture;
use crate::common::Size;
use tauri::ipc::Response;

#[tauri::command]
pub async fn snapshot() -> Response {
    let display = capture::engine::get_primary_display().await;
    let frame = capture::engine::snapshot(display).await;
    Response::new(frame.buffer)
}

#[tauri::command]
pub async fn display_size() -> Size {
    let display = capture::engine::get_primary_display().await;
    let rect = display.rect();
    Size::new(rect.size.width, rect.size.height)
}
