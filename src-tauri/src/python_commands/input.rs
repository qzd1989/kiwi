use crate::input;
use pyo3::prelude::*;

#[pyfunction]
pub fn click_left() {
    input::click_left();
}

#[pyfunction]
pub fn click_right() {
    input::click_right();
}

#[pyfunction]
pub fn press_left() {
    input::press_left();
}

#[pyfunction]
pub fn press_right() {
    input::press_right();
}

#[pyfunction]
pub fn release_left() {
    input::release_left();
}

#[pyfunction]
pub fn release_right() {
    input::release_right();
}

#[pyfunction]
pub fn move_abs(x: i32, y: i32) {
    input::move_abs(x, y);
}

#[pyfunction]
pub fn move_rel(x: i32, y: i32) {
    input::move_rel(x, y);
}

#[pyfunction]
pub fn get_location() -> (f64, f64) {
    let point = input::location();
    (point.x, point.y)
}

#[pyfunction]
pub fn scroll_vertical(length: i32) {
    input::scroll_vertical(length);
}

#[pyfunction]
pub fn scroll_horizontal(length: i32) {
    input::scroll_horizontal(length);
}
