use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}
