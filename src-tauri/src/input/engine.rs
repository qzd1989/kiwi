use super::Key;
use crate::types::Point;
use anyhow::Result;
use crossbeam_channel::{unbounded, Sender};
use device_query::{DeviceQuery as _, DeviceState};
use enigo::{
    agent::{Agent, Token},
    Axis::{Horizontal, Vertical},
    Button,
    Coordinate::{Abs, Rel},
    Direction::{Click, Press, Release},
    Enigo, Settings,
};
use std::{str::FromStr as _, sync::RwLock};
use std::{sync::LazyLock, thread};

#[derive(Debug)]
pub struct Engine {}

impl Default for Engine {
    fn default() -> Self {
        Self {}
    }
}

impl Engine {
    fn send(&self, token: &Token) {
        INPUT_SENDER.send(token.to_owned()).unwrap();
    }

    pub fn execute_token(&self, tokens: &Vec<Token>) {
        for token in tokens.iter() {
            self.send(token);
        }
    }

    pub fn click_left(&self) {
        let token = Token::Button(Button::Left, Click);
        self.send(&token);
    }

    pub fn click_right(&self) {
        let token = Token::Button(Button::Right, Click);
        self.send(&token);
    }

    pub fn press_left(&self) {
        let token = Token::Button(Button::Left, Press);
        self.send(&token);
    }

    pub fn press_right(&self) {
        let token = Token::Button(Button::Right, Press);
        self.send(&token);
    }

    pub fn release_left(&self) {
        let token = Token::Button(Button::Left, Release);
        self.send(&token);
    }

    pub fn release_right(&self) {
        let token = Token::Button(Button::Right, Release);
        self.send(&token);
    }

    pub fn move_absolute(&self, point: &Point) {
        let token = Token::MoveMouse(point.x, point.y, Abs);
        self.send(&token);
    }

    pub fn move_relative(&self, point: &Point) {
        let token = Token::MoveMouse(point.x, point.y, Rel);
        self.send(&token);
    }

    pub fn get_mouse_location(&self) -> Point {
        let mouse = DEVICE_STATE.read().unwrap().get_mouse();
        let location = mouse.coords;
        Point::new(location.0, location.1)
    }

    pub fn scroll_vertical(&self, length: i32) {
        let token = Token::Scroll(length, Vertical);
        self.send(&token);
    }

    pub fn scroll_horizontal(&self, length: i32) {
        let token = Token::Scroll(length, Horizontal);
        self.send(&token);
    }

    pub fn press_key(&self, key: impl AsRef<str>) -> Result<()> {
        let key = Key::from_str(key.as_ref())?;
        let token = Token::Key(key.into(), Press);
        self.send(&token);
        Ok(())
    }

    pub fn release_key(&self, key: impl AsRef<str>) -> Result<()> {
        let key = Key::from_str(key.as_ref())?;
        let token = Token::Key(key.into(), Release);
        self.send(&token);
        Ok(())
    }

    pub fn click_key(&self, key: impl AsRef<str>) -> Result<()> {
        let key = Key::from_str(key.as_ref())?;
        let token = Token::Key(key.into(), Click);
        self.send(&token);
        Ok(())
    }

    pub fn input_text(&self, text: impl AsRef<str>) {
        let token = Token::Text(text.as_ref().to_string());
        self.send(&token);
    }
}

/// A temporary way to run enigo in sub-thread on macos
/// https://github.com/enigo-rs/enigo/issues/436
static INPUT_SENDER: LazyLock<Sender<Token>> = LazyLock::new(|| {
    let (tx, rv) = unbounded::<Token>();
    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        while let Ok(token) = rv.recv() {
            if let Err(error) = enigo.execute(&token) {
                println!("{:?}", error);
            }
        }
    });
    tx
});

static DEVICE_STATE: LazyLock<RwLock<DeviceState>> =
    LazyLock::new(|| RwLock::new(DeviceState::new()));
