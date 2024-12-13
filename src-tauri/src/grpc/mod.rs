mod find;
mod server;
use crate::common::HexColor;
pub use find::*;
pub use server::{run, run_spawn};
use std::sync::LazyLock;
use tokio::runtime::Runtime;
pub mod request {
    pub type WeightPoint = (f64, f64, f64);
    pub type WeightPoints = Vec<WeightPoint>;
    pub type Point = (f64, f64);
    pub type LocatingColor = (f64, f64, Option<super::HexColor>);
    pub type LocatingColors = Vec<LocatingColor>;
    pub type HexColors = Vec<super::HexColor>;
    pub type Langs = Vec<String>;
}
pub mod response {
    pub type WeightPoint = (i32, i32, f64);
    pub type WeightPoints = Vec<WeightPoint>;
    pub type Point = (i32, i32);
    pub type LocatingColor = (i32, i32, Option<super::HexColor>);
    pub type LocatingColors = Vec<LocatingColor>;
    pub type HexColors = Vec<super::HexColor>;
    pub type Langs = Vec<String>;
    pub const EMPTY_WEIGHT_POINT: WeightPoint = (-1, -1, 0.0);
    pub const EMPTY_WEIGHT_POINTS: Vec<WeightPoint> = Vec::new();
    pub const EMPTY_LOCATING_COLORS: LocatingColors = Vec::new();
    pub const EMPTY_LOCATING_COLOR: LocatingColor = (-1, -1, None);
}

const ADDR: &str = "[::1]:50555";
pub static RUN_TIME: LazyLock<Runtime> = LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());
