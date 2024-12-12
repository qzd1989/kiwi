mod find;
mod server;

use crate::common::HexColor;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub use find::*;
pub use server::{run, run_spawn};

const ADDR: &str = "[::1]:50555";
pub static RUN_TIME: LazyLock<Runtime> = LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

pub type WeightPoint = (i32, i32, f64);
pub type WeightPoints = Vec<WeightPoint>;
pub type Point = (i32, i32);
pub type LocatingColor = (i32, i32, Option<HexColor>);
pub type LocatingColors = Vec<LocatingColor>;
pub type HexColors = Vec<HexColor>;
pub type Langs = Vec<String>;

const EMPTY_WEIGHT_POINT: WeightPoint = (-1, -1, 0.0);
const EMPTY_WEIGHT_POINTS: Vec<WeightPoint> = Vec::new();
const EMPTY_LOCATING_COLORS: LocatingColors = Vec::new();
const EMPTY_LOCATING_COLOR: LocatingColor = (-1, -1, None);
// const EMPTY_TEXT: &str = "";
// const EMPTY_POINT: Point = (-1, -1);
