use super::utils::{
    get_image_args, get_required_hex_colors, get_required_hexcolor, get_required_point,
    get_required_relative_points, get_required_rgb_offset,
};
use crate::{app::App, commands::websocket::utils::get_required_string, types::Size};
use anyhow::Result;
use serde_json::Value;

pub fn find_image(args: Value) -> Result<Value> {
    let (template, start_point, size, threshold) = get_image_args(&args)?;
    let frame = App::get_frame()?;
    let result = frame.find_image(&template, start_point, size, threshold)?;
    Ok(serde_json::to_value(result)?)
}

pub fn find_images(args: Value) -> Result<Value> {
    let (template, start_point, size, threshold) = get_image_args(&args)?;
    let frame = App::get_frame()?;
    let result = frame.find_images(&template, start_point, size, threshold)?;
    Ok(serde_json::to_value(result)?)
}

pub fn find_relative_colors(args: Value) -> Result<Value> {
    let vertex_hex = get_required_hexcolor(&args, "vertex_hex")?;
    let relative_points = get_required_relative_points(&args, "relative_points")?;
    let start_point = get_required_point(&args, "start_point")?;
    let end_point = get_required_point(&args, "end_point")?;
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let rgb_offset = get_required_rgb_offset(&args, "rgb_offset")?;
    let frame = App::get_frame()?;
    let result =
        frame.find_relative_colors(vertex_hex, relative_points, start_point, size, rgb_offset)?;
    Ok(serde_json::to_value(result)?)
}

pub fn find_colors(args: Value) -> Result<Value> {
    let hex_colors = get_required_hex_colors(&args, "hex_colors")?;
    let start_point = get_required_point(&args, "start_point")?;
    let end_point = get_required_point(&args, "end_point")?;
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let rgb_offset = get_required_rgb_offset(&args, "rgb_offset")?;
    let frame = App::get_frame()?;
    let result = frame.find_colors(&hex_colors, start_point, size, rgb_offset)?;
    Ok(serde_json::to_value(result)?)
}

pub fn recognize_text(args: Value) -> Result<Value> {
    let start_point = get_required_point(&args, "start_point")?;
    let end_point = get_required_point(&args, "end_point")?;
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let frame = App::get_frame()?;
    let result = frame.recognize_text(start_point, size)?;
    Ok(serde_json::to_value(result)?)
}

pub fn save(args: Value) -> Result<Value> {
    let path = get_required_string(&args, "path")?;
    let frame = App::get_frame()?;
    let result = frame.save(&path)?;
    Ok(serde_json::to_value(result)?)
}
