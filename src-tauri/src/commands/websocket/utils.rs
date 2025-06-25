use anyhow::{anyhow, Result};
use image::{ImageReader, RgbaImage};
use serde_json::Value;
use std::path::{Path, PathBuf};

use crate::{
    app::App,
    types::{HexColor, Point, RelativePoint, RgbOffset, Size},
};

pub fn get_required_string(args: &Value, key: &str) -> Result<String> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_i32(args: &Value, key: &str) -> Result<i32> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_f64(args: &Value, key: &str) -> Result<f64> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_hexcolor(args: &Value, key: &str) -> Result<HexColor> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_point(args: &Value, key: &str) -> Result<Point> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_rgb_offset(args: &Value, key: &str) -> Result<RgbOffset> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_relative_points(args: &Value, key: &str) -> Result<Vec<RelativePoint>> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_required_hex_colors(args: &Value, key: &str) -> Result<Vec<HexColor>> {
    args.get(key)
        .ok_or_else(|| anyhow!("{} not found", key))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| anyhow!("parse {} error: {:?}", key, e))
        })
}

pub fn get_project_dir() -> Result<PathBuf> {
    App::try_with_project(|project| project.path.clone())
}

pub fn load_template_image(project_dir: &Path, subpath: &str) -> Result<RgbaImage> {
    let template_path = project_dir
        .join("data")
        .join("images")
        .join(format!("{}.png", subpath));
    if !template_path.exists() {
        return Err(anyhow!("{:?} does not exist", template_path));
    }
    Ok(ImageReader::open(&template_path)?.decode()?.into_rgba8())
}

pub fn get_image_args(args: &Value) -> Result<(RgbaImage, Point, Size, f64)> {
    let project_dir = get_project_dir()?;
    let subpath = get_required_string(args, "subpath")?;
    let template_buffer = load_template_image(&project_dir, &subpath)?;

    let start_point = get_required_point(args, "start_point")?;
    let end_point = get_required_point(args, "end_point")?;
    let threshold = get_required_f64(args, "threshold")?;
    let size = Size::new_from_start_end_points(start_point, end_point)?;

    Ok((template_buffer, start_point, size, threshold))
}
