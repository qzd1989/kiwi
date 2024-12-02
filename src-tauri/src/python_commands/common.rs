use crate::common::{LocatingColor, Point};
use pyo3::prelude::*;

#[pymethods]
impl Point {
    #[new]
    pub fn py_new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[pymethods]
impl LocatingColor {
    #[new]
    pub fn py_new(point: Point, hex: String) -> Self {
        Self { point, hex }
    }
}
