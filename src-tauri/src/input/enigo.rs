use crate::common::Point;
use enigo::{
    Axis::{Horizontal, Vertical},
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Mouse, Settings,
};
use std::thread;
use std::time::Duration;

pub fn click_left() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Left, Click).unwrap();
}

pub fn click_right() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Right, Click).unwrap();
}

pub fn press_left() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Left, Press).unwrap();
}
pub fn press_right() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Right, Press).unwrap();
}

pub fn release_left() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Left, Release).unwrap();
}
pub fn release_right() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Right, Release).unwrap();
}

pub fn move_abs(x: i32, y: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
}

pub fn move_rel(x: i32, y: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.move_mouse(x, y, Coordinate::Rel).unwrap();
}

pub fn location() -> Point {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let (x, y) = enigo.location().unwrap();
    Point::new(x, y)
}

pub fn sleep(time: u64) {
    let wait_time = Duration::from_millis(time);
    thread::sleep(wait_time);
}

pub fn scroll_vertical(length: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.scroll(length, Vertical).unwrap();
}

pub fn scroll_horizontal(length: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.scroll(length, Horizontal).unwrap();
}
