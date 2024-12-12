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
pub fn move_abs(x: f64, y: f64) {
    input::move_abs(x as i32, y as i32);
}

#[pyfunction]
pub fn move_rel(x: f64, y: f64) {
    input::move_rel(x as i32, y as i32);
}

#[pyfunction]
pub fn get_location() -> (f64, f64) {
    let point = input::location();
    (point.x, point.y)
}

#[pyfunction]
pub fn scroll_vertical(length: f64) {
    input::scroll_vertical(length as i32);
}

#[pyfunction]
pub fn scroll_horizontal(length: f64) {
    input::scroll_horizontal(length as i32);
}

#[pyfunction]
pub fn sleep(time: f64) {
    std::thread::sleep(std::time::Duration::from_millis(time as u64));
}
