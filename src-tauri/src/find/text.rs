use anyhow::{Ok, Result};

use crate::capture::Frame;

pub fn recognize(
    frame: Frame,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
) -> Result<String> {
    //todo
    Ok("123".to_string())
}
