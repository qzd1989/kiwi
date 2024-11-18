use crate::common::Point;
use pyo3::prelude::*;
pub mod input;

pub fn input_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let input = PyModule::new(parent.py(), "input")?;
    input.add_function(wrap_pyfunction!(input::click_left, &input)?)?;
    input.add_function(wrap_pyfunction!(input::location, &input)?)?;
    input.add_class::<Point>()?;
    parent.add_submodule(&input)
}
