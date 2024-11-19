use crate::{common::Point, input};
use pyo3::prelude::*;

#[pyfunction]
pub fn click_left() -> PyResult<()> {
    input::click_left();
    Ok(())
}

#[pyfunction]
pub fn click_right() -> PyResult<()> {
    input::click_right();
    Ok(())
}

#[pyfunction]
pub fn press_left() -> PyResult<()> {
    input::press_left();
    Ok(())
}

#[pyfunction]
pub fn press_right() -> PyResult<()> {
    input::press_right();
    Ok(())
}

#[pyfunction]
pub fn release_left() -> PyResult<()> {
    input::release_left();
    Ok(())
}

#[pyfunction]
pub fn release_right() -> PyResult<()> {
    input::release_right();
    Ok(())
}

#[pyfunction]
pub fn move_abs(x: i32, y: i32) -> PyResult<()> {
    input::move_abs(x, y);
    Ok(())
}

#[pyfunction]
pub fn move_rel(x: i32, y: i32) -> PyResult<()> {
    input::move_rel(x, y);
    Ok(())
}

#[pyfunction]
pub fn location() -> PyResult<Point> {
    Ok(input::location())
}

#[pyfunction]
pub fn scroll_vertical(length: i32) -> PyResult<()> {
    input::scroll_vertical(length);
    Ok(())
}

#[pyfunction]
pub fn scroll_horizontal(length: i32) -> PyResult<()> {
    input::scroll_horizontal(length);
    Ok(())
}
