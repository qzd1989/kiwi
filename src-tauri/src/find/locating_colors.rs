use super::common::{crop_rgba, frame_to_rgba, hex_to_rgb};
use crate::{
    capture::Frame,
    common::{LocatingColor, Point, RgbColor, Size},
};
use image::{ImageBuffer, Rgba};
use std::cmp::{max, min};

pub fn find_one(
    frame: Frame,
    locating_colors: Vec<LocatingColor>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
    offset_r: u8,
    offset_g: u8,
    offset_b: u8,
) -> Option<Point> {
    let (x, y) = (x.into() as u32, y.into() as u32);
    // 1. get peak point
    let peak = find_peak(&locating_colors);
    let peak_rgb = hex_to_rgb(&peak.hex);
    // 2. get relatives except peak
    let relatives = get_relatives(&locating_colors, &peak);
    // 3. get rect size of locating_colors
    let size = get_size(&relatives, &peak);
    // 3. scan and match peak point.
    let buffer = frame_to_rgba(frame).unwrap();
    let buffer = crop_rgba(&buffer, x, y, width, height);
    let (width, height) = buffer.dimensions();
    for y in 0..height {
        for x in 0..width {
            // 当剩余高度小于定位点尺寸高度,停止继续匹配.
            if height - y < size.height as u32 {
                return None;
            }
            let pixel = buffer.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let rgb = RgbColor(r, g, b);
            if let Some(offsets) = peak_rgb.range_compare(offset_r, offset_g, offset_b, &rgb) {
                let origin_abs_point = Point::new(x as f64, y as f64);
                // 4. compare others pixel point.
                if match_relatives(&buffer, &origin_abs_point, &relatives, &offsets) {
                    return Some(peak.point);
                }
            }
        }
    }
    None
}

pub fn find_peak(locating_colors: &Vec<LocatingColor>) -> LocatingColor {
    let sort = |locating_colors: &Vec<LocatingColor>| -> Vec<LocatingColor> {
        let mut need_to_sort = locating_colors.clone();
        need_to_sort.sort_by(|a, b| {
            if a.point.y == b.point.y {
                a.point
                    .x
                    .partial_cmp(&b.point.x)
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                a.point
                    .y
                    .partial_cmp(&b.point.y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        return need_to_sort;
    };
    let sorted = sort(locating_colors);
    sorted.first().unwrap().clone()
}

fn match_relatives(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    origin_abs_point: &Point,
    relatives: &Vec<LocatingColor>,
    offsets: &(i16, i16, i16),
) -> bool {
    let mut colors = relatives.to_owned();
    let convert_to_abs_point = |relative_point: &mut Point, origin_abs_point: &Point| -> Point {
        relative_point.x += origin_abs_point.x;
        relative_point.y += origin_abs_point.y;
        relative_point.to_owned()
    };
    let convert_to_offset_rgb =
        |relative_hex: &mut str, offsets: &(i16, i16, i16)| -> (u8, u8, u8) {
            let RgbColor(r, g, b) = hex_to_rgb(&relative_hex);
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

fn get_size(relatives: &Vec<LocatingColor>, peak: &LocatingColor) -> Size {
    let mut start_x = peak.point.x;
    let mut start_y = peak.point.y;
    let mut end_x = peak.point.x;
    let mut end_y = peak.point.y;
    for relative in relatives.iter() {
        start_x = min(start_x as i32, relative.point.x as i32) as f64;
        start_y = min(start_y as i32, relative.point.y as i32) as f64;
        end_x = max(end_x as i32, relative.point.x as i32) as f64;
        end_y = max(end_y as i32, relative.point.y as i32) as f64;
    }
    let width = start_x.abs() + end_x.abs();
    let height = start_y.abs() + start_y.abs();
    Size::new(width, height)
}

fn get_relatives(locating_colors: &Vec<LocatingColor>, peak: &LocatingColor) -> Vec<LocatingColor> {
    let mut colors = Vec::new();
    let mut peak_index = 0;
    for (index, locating_color) in locating_colors.iter().enumerate() {
        if locating_color == peak {
            peak_index = index;
            break;
        }
    }
    for (index, locating_color) in locating_colors.iter().enumerate() {
        if index == peak_index {
            continue;
        }
        colors.push(LocatingColor::new(
            Point::new(
                locating_color.point.x - peak.point.x,
                locating_color.point.y - peak.point.y,
            ),
            locating_color.hex.clone(),
        ));
    }
    colors
}
