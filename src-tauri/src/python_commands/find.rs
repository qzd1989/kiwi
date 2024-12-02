use crate::common::{HexColor, LocatingColor};
use pyo3::prelude::*;

#[pyfunction]
pub fn find_image(path: String, start: (i32, i32), end: (i32, i32), threshold: f64) -> (f64, f64) {
    (-1.0, -1.0)
}

#[pyfunction]
pub fn find_images(
    path: String,
    start: (i32, i32),
    end: (i32, i32),
    threshold: f64,
) -> Vec<(f64, f64)> {
    let vec = Vec::new();
    vec
}

#[pyfunction]
pub fn global_find_image(path: String, threshold: f64) -> (f64, f64) {
    (-1.0, -1.0)
}

#[pyfunction]
pub fn global_find_images(path: String, threshold: f64) -> Vec<(f64, f64)> {
    let vec = Vec::new();
    vec
}

#[pyfunction]
pub fn find_locating_color(
    colors: Vec<LocatingColor>,
    start: (i32, i32),
    end: (i32, i32),
    offsets: (u8, u8, u8),
) -> (f64, f64) {
    (-1.0, -1.0)
}

#[pyfunction]
pub fn global_find_locating_color(colors: Vec<LocatingColor>, offsets: (u8, u8, u8)) -> (f64, f64) {
    (-1.0, -1.0)
}

#[pyfunction]
pub fn find_color(
    colors: Vec<HexColor>,
    start: (i32, i32),
    end: (i32, i32),
    offsets: (u8, u8, u8),
) -> (f64, f64) {
    (-1.0, -1.0)
}

#[pyfunction]
pub fn global_find_color(colors: Vec<HexColor>, offsets: (u8, u8, u8)) -> (f64, f64) {
    (-1.0, -1.0)
}

#[pyfunction]
pub fn recognize_text(start: (i32, i32), end: (i32, i32), languages: Vec<String>) -> String {
    "".to_string()
}
