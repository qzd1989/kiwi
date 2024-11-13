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
    let (x, y) = (x.into() as u32, y.into() as u32);
    let buffer = frame_to_rgba(frame).unwrap();
    let buffer = crop_rgba(&buffer, x, y, width, height);
    let (width, height) = buffer.dimensions();
    true
}
