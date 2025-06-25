use super::{HexColor, Point};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RelativePoint {
    pub point: Point,
    pub hex: HexColor,
}
impl RelativePoint {
    pub fn new(point: Point, hex: String) -> Self {
        Self { point, hex }
    }
}
