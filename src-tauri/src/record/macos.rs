use crate::{
    input::{Abs, Button, Engine as InputEngine, Horizontal, Key, Press, Release, Token, Vertical},
    types::Point,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::AppHandle;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

pub struct Engine {
    child: Arc<Mutex<Option<CommandChild>>>,
    running: Arc<AtomicBool>,
}

impl Engine {
    pub fn start<F>(&mut self, app: &AppHandle, callback: F)
    where
        F: 'static + Fn(Vec<Token>) + Send,
    {
        self.stop();
        let command = app.shell().sidecar("event_listener").unwrap();
        let (mut rx, child) = command.arg("run").spawn().expect("Failed to spawn sidecar");
        self.child = Arc::new(Mutex::new(Some(child)));
        let running = Arc::clone(&self.running);
        tauri::async_runtime::spawn(async move {
            while let Some(command_event) = rx.recv().await {
                if let CommandEvent::Stdout(line_bytes) = command_event {
                    running.store(true, Ordering::SeqCst);
                    let line = String::from_utf8_lossy(&line_bytes);
                    let data = line.into_owned();
                    if let Ok(event) = serde_json::from_str::<Event>(&data) {
                        if let Ok(tokens) = Vec::<Token>::try_from(event) {
                            callback(tokens);
                        }
                    }
                }
            }
            running.store(false, Ordering::SeqCst);
        });
        self.explicit_confirm_engine_is_started();
    }

    pub fn stop(&self) {
        let mut guard = self.child.lock().unwrap();
        if let Some(mut child) = guard.take() {
            child.write("stop".as_bytes()).unwrap();
        }
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn explicit_confirm_engine_is_started(&self) {
        let running = Arc::clone(&self.running);
        thread::spawn(move || {
            let mut x = -2;
            for _ in 0..1000 {
                if running.load(Ordering::SeqCst) {
                    return;
                }
                x = -x;
                let point = Point::new(x, 0);
                InputEngine::move_relative(&point);
                thread::sleep(Duration::from_millis(10));
            }
            eprintln!("Warning: input engine did not confirm startup within expected time.");
        });
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "event")]
enum Event {
    #[serde(rename = "move")]
    Move { x: f64, y: f64 },

    #[serde(rename = "click")]
    Click {
        action: String,
        x: f64,
        y: f64,
        button: String,
    },

    #[serde(rename = "key_press")]
    KeyPress { key: String },

    #[serde(rename = "key_release")]
    KeyRelease { key: String },

    #[serde(rename = "scroll")]
    Scroll {
        direction: String,
        x: f64,
        y: f64,
        dx: i32,
        dy: i32,
    },
}

impl TryFrom<Event> for Vec<Token> {
    type Error = anyhow::Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        let mut tokens = Vec::new();
        match value {
            Event::Move { x, y } => {
                let (x, y) = (x as i32, y as i32);
                tokens.push(Token::MoveMouse(x, y, Abs));
            }
            Event::Click {
                action,
                x,
                y,
                button,
            } => {
                let (x, y) = (x as i32, y as i32);
                tokens.push(Token::MoveMouse(x, y, Abs));
                match (action.as_str(), button.as_str()) {
                    ("pressed", "Button.left") => {
                        tokens.push(Token::Button(Button::Left, Press));
                    }
                    ("pressed", "Button.right") => {
                        tokens.push(Token::Button(Button::Right, Press));
                    }
                    ("released", "Button.left") => {
                        tokens.push(Token::Button(Button::Left, Release));
                    }
                    ("released", "Button.right") => {
                        tokens.push(Token::Button(Button::Right, Release));
                    }
                    (a, b) => {
                        return Err(anyhow!("unsupported click type: action={} button={}", a, b))
                    }
                }
            }
            Event::KeyPress { key } => {
                if let Some(key) = convert_key(&key) {
                    tokens.push(Token::Key(key, Press));
                }
            }
            Event::KeyRelease { key } => {
                if let Some(key) = convert_key(&key) {
                    tokens.push(Token::Key(key, Release));
                }
            }
            Event::Scroll { x, y, dx, dy, .. } => {
                let (x, y) = (x as i32, y as i32);
                tokens.push(Token::MoveMouse(x, y, Abs));
                if dx != 0 {
                    tokens.push(Token::Scroll(dx, Horizontal));
                }
                if dy != 0 {
                    tokens.push(Token::Scroll(dy, Vertical));
                }
            }
        };
        Ok(tokens)
    }
}

fn convert_key(key: &str) -> Option<Key> {
    let key = key.to_lowercase();
    let converted = match key.as_str() {
        "key.esc" => Key::Escape,
        "key.f1" => Key::F1,
        "key.f2" => Key::F2,
        "key.f3" => Key::F3,
        "key.f4" => Key::F4,
        "key.f5" => Key::F5,
        "key.f6" => Key::F6,
        "key.f7" => Key::F7,
        "key.f8" => Key::F8,
        "key.f9" => Key::F9,
        "key.f10" => Key::F10,
        "key.f11" => Key::F11,
        "key.f12" => Key::F12,
        "key.backspace" => Key::Backspace,
        "key.tab" => Key::Tab,
        "key.caps_lock" => Key::CapsLock,
        "key.enter" => Key::Return,
        "key.shift" => Key::Shift,
        "key.shift_r" => Key::Shift,
        "key.ctrl" => Key::Control,
        "key.cmd" => Key::Meta,
        "key.alt" => Key::Alt,
        "key.space" => Key::Space,
        "key.alt_r" => Key::Alt,
        "key.cmd_r" => Key::Meta,
        "key.ctrl_r" => Key::Control,
        #[cfg(target_os = "windows")]
        "\\u0005" => Key::Insert,
        "key.delete" => Key::Delete,
        "key.home" => Key::Home,
        "key.end" => Key::End,
        "key.page_up" => Key::PageUp,
        "key.page_down" => Key::PageDown,
        "key.up" => Key::UpArrow,
        "key.down" => Key::DownArrow,
        "key.left" => Key::LeftArrow,
        "key.right" => Key::RightArrow,
        "`" => Key::Unicode('`'),
        "1" => Key::Unicode('1'),
        "2" => Key::Unicode('2'),
        "3" => Key::Unicode('3'),
        "4" => Key::Unicode('4'),
        "5" => Key::Unicode('5'),
        "6" => Key::Unicode('6'),
        "7" => Key::Unicode('7'),
        "8" => Key::Unicode('8'),
        "9" => Key::Unicode('9'),
        "0" => Key::Unicode('0'),
        "-" => Key::Unicode('-'),
        "=" => Key::Unicode('='),
        "q" => Key::Unicode('q'),
        "w" => Key::Unicode('w'),
        "e" => Key::Unicode('e'),
        "r" => Key::Unicode('r'),
        "t" => Key::Unicode('t'),
        "y" => Key::Unicode('y'),
        "u" => Key::Unicode('u'),
        "i" => Key::Unicode('i'),
        "o" => Key::Unicode('o'),
        "p" => Key::Unicode('p'),
        "[" => Key::Unicode('['),
        "]" => Key::Unicode(']'),
        "\\" => Key::Unicode('\\'),
        "a" => Key::Unicode('a'),
        "s" => Key::Unicode('s'),
        "d" => Key::Unicode('d'),
        "f" => Key::Unicode('f'),
        "g" => Key::Unicode('g'),
        "h" => Key::Unicode('h'),
        "j" => Key::Unicode('j'),
        "k" => Key::Unicode('k'),
        "l" => Key::Unicode('l'),
        ";" => Key::Unicode(';'),
        "'" => Key::Unicode('\''),
        "z" => Key::Unicode('z'),
        "x" => Key::Unicode('x'),
        "c" => Key::Unicode('c'),
        "v" => Key::Unicode('v'),
        "b" => Key::Unicode('b'),
        "n" => Key::Unicode('n'),
        "m" => Key::Unicode('m'),
        "," => Key::Unicode(','),
        "." => Key::Unicode('.'),
        "/" => Key::Unicode('/'),
        _ => return None,
    };
    Some(converted)
}
