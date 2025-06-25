use super::error::CommandResult;
use crate::{
    app::App,
    types::{HexColor, Point, RelativePoint, RgbOffset},
};

#[tauri::command]
pub fn generate_find_image_code(
    subpath: String,
    start_point: Point,
    end_point: Point,
    threshold: f64,
) -> CommandResult<String> {
    let code = App::try_with_project(|project| project.interpreter.get_code())?;
    Ok(code.find_image(&subpath, &start_point, &end_point, threshold))
}

#[tauri::command]
pub fn generate_find_images_code(
    subpath: String,
    start_point: Point,
    end_point: Point,
    threshold: f64,
) -> CommandResult<String> {
    let code = App::try_with_project(|project| project.interpreter.get_code())?;
    Ok(code.find_images(&subpath, &start_point, &end_point, threshold))
}

#[tauri::command]
pub fn generate_find_relative_colors_code(
    vertex_hex: HexColor,
    relative_points: Vec<RelativePoint>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<String> {
    let code = App::try_with_project(|project| project.interpreter.get_code())?;
    Ok(code.find_relative_colors(
        &vertex_hex,
        &relative_points,
        &start_point,
        &end_point,
        &rgb_offset,
    ))
}

#[tauri::command]
pub fn generate_find_colors_code(
    hex_colors: Vec<HexColor>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<String> {
    let code = App::try_with_project(|project| project.interpreter.get_code())?;
    Ok(code.find_colors(&hex_colors, &start_point, &end_point, &rgb_offset))
}

#[tauri::command]
pub fn generate_recognize_text_code(start_point: Point, end_point: Point) -> CommandResult<String> {
    let code = App::try_with_project(|project| project.interpreter.get_code())?;
    Ok(code.recognize_text(&start_point, &end_point))
}
