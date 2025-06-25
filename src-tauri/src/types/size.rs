use super::Point;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
impl Size {
    pub fn new(width: impl Into<u32>, height: impl Into<u32>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
    pub fn new_from_start_end_points(start_point: Point, end_point: Point) -> Result<Self> {
        let width = end_point.x - start_point.x;
        let height = end_point.y - start_point.y;
        if width <= 0 {
            return Err(anyhow!("width must bigger than zero"));
        }
        if height <= 0 {
            return Err(anyhow!("height must bigger than zero"));
        }
        let (width, height) = (width as u32, height as u32);
        Ok(Self { width, height })
    }
}
