use crate::common::{LocatingColor, WeightPoint};
use serde_json::json;

#[tauri::command(rename_all = "snake_case")]
pub fn find_image(
    origin_base64: String,
    captured_base64: String,
    start_at: String,
    end_at: String,
    threshold: f64,
) -> Vec<WeightPoint> {
    //todo
    todo!()
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
