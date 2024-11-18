use crate::input;
use pyo3::prelude::*;

#[pyfunction]
pub fn click_left() -> PyResult<()> {
    input::click_left();
    Ok(())
}
