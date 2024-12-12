mod find;
mod server;

use crate::common::HexColor;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub use find::*;
pub use server::{run, run_spawn};

const ADDR: &str = "[::1]:50555";
pub static RUN_TIME: LazyLock<Runtime> = LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

pub type WeightPoint = (f64, f64, f64);
pub type WeightPoints = Vec<WeightPoint>;
pub type Point = (f64, f64);
pub type LocatingColor = (f64, f64, Option<HexColor>);
pub type LocatingColors = Vec<LocatingColor>;
pub type HexColors = Vec<HexColor>;
pub type Langs = Vec<String>;

const EMPTY_WEIGHT_POINT: WeightPoint = (-1.0, -1.0, 0.0);
const EMPTY_WEIGHT_POINTS: Vec<WeightPoint> = Vec::new();
const EMPTY_LOCATING_COLORS: LocatingColors = Vec::new();
const EMPTY_LOCATING_COLOR: LocatingColor = (-1.0, -1.0, None);
// const EMPTY_TEXT: &str = "";
// const EMPTY_POINT: Point = (-1.0, -1.0);
