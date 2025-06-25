use super::{Point, Weight};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct WeightPoint {
    pub point: Point,
    pub weight: Weight,
}
impl WeightPoint {
    pub fn new(point: Point, weight: Weight) -> Self {
        Self { point, weight }
    }
}
