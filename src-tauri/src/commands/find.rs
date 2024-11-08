use crate::{
    common::{LocatingColor, WeightPoint},
    find::{
        common::{base64_to_frame, base64_to_rgba},
        image::{find_multiple, find_one},
    },
};

#[tauri::command]
pub fn find_image(
    origin: String,
    template: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    threshold: f64,
) -> Result<WeightPoint, String> {
    let frame = base64_to_frame(&origin).unwrap();
    let template = base64_to_rgba(&template).unwrap();
    find_one(frame, template, x, y, width, height, threshold)
        .or_else(|error| Err(error.to_string()))
}

#[tauri::command]
pub fn find_images(
    origin: String,
    template: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    threshold: f64,
) -> Result<Vec<WeightPoint>, String> {
    let frame = base64_to_frame(&origin).unwrap();
    let template = base64_to_rgba(&template).unwrap();
    find_multiple(frame, template, x, y, width, height, threshold)
        .or_else(|error| Err(error.to_string()))
}

#[tauri::command]
pub fn get_peak_point(json: String) -> LocatingColor {
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
    sorted.first().unwrap().clone()
}
