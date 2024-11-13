use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct WeightPoint {
    pub point: Point,
    pub weight: f64,
}

impl WeightPoint {
    pub fn new(point: Point, weight: f64) -> Self {
        Self { point, weight }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocatingColor {
    pub point: Point,
    pub hex: String,
}

impl LocatingColor {
    pub fn new(point: Point, hex: String) -> Self {
        Self { point, hex }
    }
}

pub struct LimitedQueue<T> {
    size: usize,
    queue: VecDeque<T>,
}

impl<T> LimitedQueue<T> {
    pub fn new(size: usize) -> Self {
        LimitedQueue {
            size,
            queue: VecDeque::with_capacity(size),
        }
    }
    pub fn push(&mut self, item: T) {
        if self.queue.len() >= self.size {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.queue.get(index)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
    pub fn range_compare(
        &self,
        offset_r: u8,
        offset_g: u8,
        offset_b: u8,
        target: &RgbColor,
    ) -> Option<(i16, i16, i16)> {
        let range = (
            (self.0 - offset_r)..=(self.0 + offset_r),
            (self.1 - offset_g)..=(self.1 + offset_g),
            (self.2 - offset_b)..=(self.2 + offset_b),
        );
        if range.0.contains(&target.0) && range.1.contains(&target.1) && range.2.contains(&target.2)
        {
            return Some((
                self.0 as i16 - target.0 as i16,
                self.1 as i16 - target.1 as i16,
                self.2 as i16 - target.2 as i16,
            ));
        } else {
            None
        }
    }
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    pub fn to_u32(&self) -> u32 {
        ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }
}
