use crate::{
    common::{Base64PngExt, LocatingColor, Point, WeightPoint},
    find::{image, locating_color},
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
    let frame = origin.to_frame().unwrap();
    let template = template.to_buffer().unwrap();
    image::find_one(frame, template, x, y, width, height, threshold)
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
    let frame = origin.to_frame().unwrap();
    let template = template.to_buffer().unwrap();
    image::find_multiple(frame, template, x, y, width, height, threshold)
        .or_else(|error| Err(error.to_string()))
}

#[tauri::command]
pub fn find_peak(locating_colors: String) -> LocatingColor {
    let locating_colors: Vec<LocatingColor> = serde_json::from_str(&locating_colors).unwrap();
    locating_color::find_peak(&locating_colors)
}

#[tauri::command]
pub fn find_locating_color(
    origin: String,
    locating_colors: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offset_r: u8,
    offset_g: u8,
    offset_b: u8,
) -> Result<Point, String> {
    let frame = origin.to_frame().unwrap();
    let locating_colors: Vec<LocatingColor> = serde_json::from_str(&locating_colors).unwrap();
    if let Some(point) = locating_color::find_one(
        frame,
        locating_colors,
        x,
        y,
        width,
        height,
        offset_r,
        offset_g,
        offset_b,
    ) {
        return Ok(point);
    }
    return Err("point is not found".to_string());
}
