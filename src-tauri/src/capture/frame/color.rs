use super::Frame;
use crate::{
    extensions::ImageBufferRgbaExt as _,
    types::{HexColor, HexColorExt as _, Point, RelativePoint, RgbColor, RgbOffset, Size},
};
use anyhow::Result;
use image::{ImageBuffer, Rgba};

impl Frame {
    pub fn find_colors(
        &self,
        hex_colors: &[HexColor],
        point: Point,
        size: Size,
        rgb_offset: RgbOffset,
    ) -> Result<Vec<RelativePoint>> {
        let buffer = self.to_buffer()?.crop(point, size);
        let (width, height) = buffer.dimensions();
        let mut rgb_colors: Vec<RgbColor> = hex_colors
            .iter()
            .map(|hex_color| hex_color.to_rgb())
            .collect();
        let mut locating_colors = Vec::new();
        for cropped_y in 0..height {
            for cropped_x in 0..width {
                let pixel = buffer.get_pixel(cropped_x, cropped_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let rgb = RgbColor(r, g, b);
                rgb_colors.retain(|rgb_color| {
                    if let Some(_) = rgb_color.range_compare(rgb_offset, &rgb) {
                        locating_colors.push(RelativePoint::new(
                            Point::new(cropped_x as i32 + point.x, cropped_y as i32 + point.y),
                            rgb_color.to_hex(),
                        ));
                        return false;
                    }
                    return true;
                });
            }
        }
        if !rgb_colors.is_empty() {
            // incomplete match color
            locating_colors.clear();
            return Ok(locating_colors);
        }
        Ok(locating_colors)
    }

    pub fn has_colors(
        &self,
        colors: &[HexColor],
        point: Point,
        size: Size,
        rgb_offset: RgbOffset,
    ) -> Result<bool> {
        Ok(self.find_colors(colors, point, size, rgb_offset).is_ok())
    }

    pub fn find_relative_colors(
        &self,
        vertex_hex: HexColor,                //peak point is {x:0, y:0}
        relative_points: Vec<RelativePoint>, //not include peak point
        point: Point,                        //scan start point
        size: Size,                          //scan range
        rgb_offset: RgbOffset,
    ) -> Result<Option<RelativePoint>> //return peak point
    {
        if relative_points.len() == 0 {
            let colors = vec![vertex_hex];
            return match self.find_colors(&colors, point, size, rgb_offset) {
                Ok(mut locating_colors) => Ok(locating_colors.pop()),
                Err(_) => Ok(None),
            };
        }
        let locationg_colors_rect_height =
            relative_points.iter().map(|lc| lc.point.y).max().unwrap() as u32;
        let peak_rgb = vertex_hex.to_rgb();
        let buffer = self.to_buffer().unwrap().crop(point, size);
        let (width, height) = buffer.dimensions();
        for cropped_y in 0..height {
            for cropped_x in 0..width {
                // stop matching while the remaining height is less than the cropped rectangle height.
                if height - cropped_y < locationg_colors_rect_height {
                    return Ok(None);
                }
                let pixel = buffer.get_pixel(cropped_x, cropped_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let rgb = RgbColor(r, g, b);
                if let Some(offsets) = peak_rgb.range_compare(rgb_offset, &rgb) {
                    let origin_abs_point = Point::new(cropped_x as i32, cropped_y as i32);
                    // 4. compare others pixel point.
                    if match_relatives(&buffer, &origin_abs_point, &relative_points, &offsets) {
                        let peak_abs = RelativePoint::new(
                            Point::new(cropped_x as i32 + point.x, cropped_y as i32 + point.y),
                            rgb.to_hex(),
                        );
                        return Ok(Some(peak_abs));
                    }
                }
            }
        }
        Ok(None)
    }
}

fn match_relatives(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    origin_abs_point: &Point,
    relatives: &Vec<RelativePoint>,
    offsets: &(i16, i16, i16),
) -> bool {
    let mut colors = relatives.to_owned();
    let convert_to_abs_point = |relative_point: &mut Point, origin_abs_point: &Point| -> Point {
        relative_point.x += origin_abs_point.x;
        relative_point.y += origin_abs_point.y;
        relative_point.to_owned()
    };
    let convert_to_offset_rgb =
        |relative_hex: &mut HexColor, offsets: &(i16, i16, i16)| -> (u8, u8, u8) {
            let RgbColor(r, g, b) = relative_hex.to_rgb();
            let r = (r as i16 + offsets.0) as u8;
            let g = (g as i16 + offsets.1) as u8;
            let b = (b as i16 + offsets.2) as u8;
            (r, g, b)
        };
    for relative in colors.iter_mut() {
        let point = convert_to_abs_point(&mut relative.point, origin_abs_point);
        let (offset_r, offset_g, offset_b) = convert_to_offset_rgb(&mut relative.hex, offsets);
        if let Some(pixel) = buffer.get_pixel_checked(point.x as u32, point.y as u32) {
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            if offset_r != r || offset_g != g || offset_b != b {
                return false;
            }
        }
    }
    true
}
