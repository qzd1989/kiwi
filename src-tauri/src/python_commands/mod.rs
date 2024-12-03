use crate::common::Point;
use pyo3::prelude::*;
mod common;
mod find;
mod input;

pub fn input_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let input = PyModule::new(parent.py(), "input")?;
    input.add_function(wrap_pyfunction!(input::click_left, &input)?)?;
    input.add_function(wrap_pyfunction!(input::click_right, &input)?)?;
    input.add_function(wrap_pyfunction!(input::press_left, &input)?)?;
    input.add_function(wrap_pyfunction!(input::press_right, &input)?)?;
    input.add_function(wrap_pyfunction!(input::release_left, &input)?)?;
    input.add_function(wrap_pyfunction!(input::release_right, &input)?)?;
    input.add_function(wrap_pyfunction!(input::move_abs, &input)?)?;
    input.add_function(wrap_pyfunction!(input::move_rel, &input)?)?;
    input.add_function(wrap_pyfunction!(input::get_location, &input)?)?;
    input.add_function(wrap_pyfunction!(input::scroll_vertical, &input)?)?;
    input.add_function(wrap_pyfunction!(input::scroll_horizontal, &input)?)?;
    input.add_class::<Point>()?;
    parent.add_submodule(&input)
}

pub fn find_moudle(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let find = PyModule::new(parent.py(), "find")?;
    find.add_function(wrap_pyfunction!(find::find_image, &find)?)?;
    find.add_function(wrap_pyfunction!(find::find_images, &find)?)?;
    find.add_function(wrap_pyfunction!(find::find_locating_color, &find)?)?;
    find.add_function(wrap_pyfunction!(find::find_color, &find)?)?;
    find.add_function(wrap_pyfunction!(find::find_text, &find)?)?;
    find.add_function(wrap_pyfunction!(find::global_find_image, &find)?)?;
    find.add_function(wrap_pyfunction!(find::global_find_images, &find)?)?;
    find.add_function(wrap_pyfunction!(find::global_find_locating_color, &find)?)?;
    find.add_function(wrap_pyfunction!(find::global_find_color, &find)?)?;
    find.add_function(wrap_pyfunction!(find::find_haha, &find)?)?;
    parent.add_submodule(&find)
}
