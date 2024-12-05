mod find;
mod server;

use crate::common::HexColor;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub use find::find::FindImageRequest;
pub use server::{run, run_spawn};

const ADDR: &str = "[::1]:50555";
pub static RUN_TIME: LazyLock<Runtime> = LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

const EMPTY_WEIGHT_POINT: (i32, i32, f64) = (-1, -1, 0.0);
const EMPTY_WEIGHT_POINTS: Vec<(i32, i32, f64)> = Vec::new();
const EMPTY_POINT: (i32, i32) = (-1, -1);
const EMPTY_LOCATING_COLORS: Vec<(i32, i32, HexColor)> = Vec::new();
const EMPTY_TEXT: &str = "";
