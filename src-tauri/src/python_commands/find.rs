use crate::find;
use crate::{
    capture::FRAME,
    common::{HexColor, LocatingColor, PROJECT_DIR},
    utils::fs::exists,
};
use pyo3::prelude::*;
use std::path::PathBuf;

fn project_check() -> bool {
    let project_dir = PROJECT_DIR.lock().unwrap().clone();
    match project_dir {
        Some(_) => true,
        None => false,
    }
}

fn frame_check() -> bool {
    if FRAME.lock().unwrap().is_none() {
        return false;
    }
    true
}

const EMPTY_WEIGHT_POINT: (i32, i32, f64) = (-1, -1, 0.0);
const EMPTY_WEIGHT_POINTS: Vec<(i32, i32, f64)> = Vec::new();
const EMPTY_POINT: (i32, i32) = (-1, -1);
const EMPTY_LOCATING_COLORS: Vec<(i32, i32, HexColor)> = Vec::new();
const EMPTY_TEXT: &str = "";

#[pyfunction]
pub fn get_project_dir() {}

#[pyfunction]
pub fn find_image(
    path: String, //such as "subfolder/imageFileName"
    start: (u32, u32),
    end: (u32, u32),
    threshold: f64,
) -> (i32, i32, f64) {
    if !project_check() {
        println!("project_check error");
        return EMPTY_WEIGHT_POINT;
    }
    if !frame_check() {
        println!("frame_check error");
        return EMPTY_WEIGHT_POINT;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let project_path = PROJECT_DIR.lock().unwrap().clone().unwrap();
    let project_pathbuf = PathBuf::from(project_path);
    let full_path = format!(
        "{}{}",
        project_pathbuf
            .join("resources")
            .join(path)
            .to_str()
            .unwrap()
            .to_string(),
        ".png"
    );
    println!("full path is {:?}", full_path.clone());
    if let Err(error) = exists(full_path.clone()) {
        println!("exists caused error: {}", error.to_string());
        return EMPTY_WEIGHT_POINT;
    }
    if !exists(full_path.clone()).unwrap() {
        println!("file not exist");
        return EMPTY_WEIGHT_POINT;
    }
    let (width, height) = (end.0 - start.0, end.1 - start.1);
    if width <= 0 || height <= 0 {
        println!("width ({}) or height ({}) is zero", width, height);
        return EMPTY_WEIGHT_POINT;
    }
    let template = image::open(full_path.clone()).unwrap().to_rgba8();
    match find::image::find_one(frame, template, start.0, start.1, width, height, threshold) {
        Ok(weight_point) => (
            weight_point.point.x as i32,
            weight_point.point.y as i32,
            weight_point.weight,
        ),
        Err(error) => {
            println!("find_one caused error: {}", error.to_string());
            EMPTY_WEIGHT_POINT
        }
    }
}

#[pyfunction]
pub fn find_images(
    path: String,
    start: (u32, u32),
    end: (u32, u32),
    threshold: f64,
) -> Vec<(i32, i32, f64)> {
    let mut vecs = Vec::new();
    if !project_check() {
        return EMPTY_WEIGHT_POINTS;
    }
    if !frame_check() {
        return EMPTY_WEIGHT_POINTS;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let project_path = PROJECT_DIR.lock().unwrap().clone().unwrap();
    let project_pathbuf = PathBuf::from(project_path);
    let full_path = format!(
        "{}{}",
        project_pathbuf
            .join("resources")
            .join(path)
            .to_str()
            .unwrap()
            .to_string(),
        ".png"
    );
    println!("full_path: {}", full_path.clone());
    if let Err(_) = exists(full_path.clone()) {
        return EMPTY_WEIGHT_POINTS;
    }
    if !exists(full_path.clone()).unwrap() {
        return EMPTY_WEIGHT_POINTS;
    }
    let (width, height) = (end.0 - start.0, end.1 - start.1);
    if width <= 0 || height <= 0 {
        return EMPTY_WEIGHT_POINTS;
    }
    let template = image::open(full_path.clone()).unwrap().to_rgba8();
    match find::image::find_multiple(frame, template, start.0, start.1, width, height, threshold) {
        Ok(weight_points) => {
            for weight_point in weight_points {
                vecs.push((
                    weight_point.point.x as i32,
                    weight_point.point.y as i32,
                    weight_point.weight,
                ))
            }
            vecs
        }
        Err(_) => EMPTY_WEIGHT_POINTS,
    }
}

#[pyfunction]
pub fn global_find_image(path: String, threshold: f64) -> (i32, i32, f64) {
    if !project_check() {
        return EMPTY_WEIGHT_POINT;
    }
    if !frame_check() {
        return EMPTY_WEIGHT_POINT;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (frame.width, frame.height);
    find_image(path, (0, 0), (width, height), threshold)
}

#[pyfunction]
pub fn global_find_images(path: String, threshold: f64) -> Vec<(i32, i32, f64)> {
    if !project_check() {
        return EMPTY_WEIGHT_POINTS;
    }
    if !frame_check() {
        return EMPTY_WEIGHT_POINTS;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (frame.width, frame.height);
    find_images(path, (0, 0), (width, height), threshold)
}

#[pyfunction]
pub fn find_locating_color(
    locating_colors: Vec<LocatingColor>,
    start: (u32, u32),
    end: (u32, u32),
    offsets: (u8, u8, u8),
) -> (i32, i32) {
    if !frame_check() {
        return EMPTY_POINT;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (end.0 - start.0, end.1 - start.1);
    match find::locating_color::find(
        frame,
        locating_colors,
        start.0,
        start.1,
        width,
        height,
        offsets.0,
        offsets.1,
        offsets.2,
    ) {
        Some(point) => (point.x as i32, point.y as i32),
        None => EMPTY_POINT,
    }
}

#[pyfunction]
pub fn global_find_locating_color(
    locating_colors: Vec<LocatingColor>,
    offsets: (u8, u8, u8),
) -> (i32, i32) {
    if !frame_check() {
        return EMPTY_POINT;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (frame.width, frame.height);
    find_locating_color(locating_colors, (0, 0), (width, height), offsets)
}

#[pyfunction]
pub fn find_color(
    hex_colors: Vec<HexColor>,
    start: (u32, u32),
    end: (u32, u32),
    offsets: (u8, u8, u8),
) -> Vec<(i32, i32, HexColor)> {
    if !frame_check() {
        return EMPTY_LOCATING_COLORS;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (end.0 - start.0, end.1 - start.1);
    let mut vecs = Vec::new();
    match find::color::find(
        frame, hex_colors, start.0, start.1, width, height, offsets.0, offsets.1, offsets.2,
    ) {
        Ok(locating_colors) => {
            for locating_color in locating_colors {
                vecs.push((
                    locating_color.point.x as i32,
                    locating_color.point.y as i32,
                    locating_color.hex.clone(),
                ))
            }
            vecs
        }
        Err(_) => EMPTY_LOCATING_COLORS,
    }
}

#[pyfunction]
pub fn global_find_color(
    hex_colors: Vec<HexColor>,
    offsets: (u8, u8, u8),
) -> Vec<(i32, i32, HexColor)> {
    if !frame_check() {
        return EMPTY_LOCATING_COLORS;
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (frame.width, frame.height);
    find_color(hex_colors, (0, 0), (width, height), offsets)
}

#[pyfunction]
pub fn find_text(start: (u32, u32), end: (u32, u32), langs: Vec<String>) -> String {
    if !frame_check() {
        return EMPTY_TEXT.to_string();
    }
    let frame = FRAME.lock().unwrap().clone().unwrap();
    let (width, height) = (end.0 - start.0, end.1 - start.1);
    match find::text::recognize(frame, langs, start.0, start.1, width, height) {
        Ok(text) => text,
        Err(_) => EMPTY_TEXT.to_string(),
    }
}
