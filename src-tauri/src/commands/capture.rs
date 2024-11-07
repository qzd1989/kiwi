use crate::capture;
use crate::common::{LocatingColor, Size};
use crabgrab::util::Point;
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

#[tauri::command]
pub fn get_peak_point(json: String) -> serde_json::Value {
    let locating_colors: Vec<LocatingColor> = serde_json::from_str(&json).unwrap();
    let sort = |locating_colors: Vec<LocatingColor>| -> Vec<LocatingColor> {
        let mut need_to_sort = locating_colors.clone();
        need_to_sort.sort_by(|a, b| {
            if a.point.y == b.point.y {
                a.point
                    .x
                    .partial_cmp(&b.point.x)
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                a.point
                    .y
                    .partial_cmp(&b.point.y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        return need_to_sort;
    };
    let sorted = sort(locating_colors);
    let peak_locating_color = sorted.first().unwrap();
    json!(peak_locating_color)
}

#[tauri::command(rename_all = "snake_case")]
pub fn find_image(
    origin_base64: String,
    captured_base64: String,
    start_point: String,
    end_point: String,
) -> Vec<Point> {
    //todo
    todo!()
}
