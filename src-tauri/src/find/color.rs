use crate::{capture::Frame, common::HexColor};

pub fn find_one(
    frame: Frame,
    colors: Vec<HexColor>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
    offset_r: u8,
    offset_g: u8,
    offset_b: u8,
) -> bool {
    true
}
