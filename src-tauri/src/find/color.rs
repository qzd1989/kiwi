use crate::{
    capture::Frame,
    common::{HexColor, HexColorExt, ImageBufferRgbaExt as _, LocatingColor, Point, RgbColor},
};
use anyhow::{anyhow, Ok, Result};

pub fn find(
    frame: Frame,
    colors: Vec<HexColor>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
    offset_r: u8,
    offset_g: u8,
    offset_b: u8,
) -> Result<Vec<LocatingColor>> {
    let (x, y) = (x.into() as u32, y.into() as u32);
    let buffer = frame.to_buffer().unwrap().crop(x, y, width, height);
    let (width, height) = buffer.dimensions();
    let mut rgb_colors: Vec<RgbColor> = colors.iter().map(|hex_color| hex_color.to_rgb()).collect();
    let mut locating_colors = Vec::new();
    for cropped_y in 0..height {
        for cropped_x in 0..width {
            let pixel = buffer.get_pixel(cropped_x, cropped_y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let rgb = RgbColor(r, g, b);
            rgb_colors.retain(|rgb_color| {
                if let Some(_) = rgb_color.range_compare(offset_r, offset_g, offset_b, &rgb) {
                    locating_colors.push(LocatingColor::new(
                        Point::new(cropped_x + x, cropped_y + y),
                        rgb_color.to_hex(),
                    ));
                    return false;
                }
                return true;
            });
        }
    }
    if rgb_colors.len() > 0 {
        return Err(anyhow!(format!(
            "Incomplete match : {:?}",
            rgb_colors.len(),
        )));
    }
    Ok(locating_colors)
}

pub fn has(
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
    if let Err(_) = find(
        frame, colors, x, y, width, height, offset_r, offset_g, offset_b,
    ) {
        return false;
    }
    true
}
