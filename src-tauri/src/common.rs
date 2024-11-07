use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
