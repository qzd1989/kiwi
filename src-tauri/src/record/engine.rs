use crate::input;
use anyhow::Result;
use enigo::agent::Token;
#[cfg(target_os = "macos")]
use rdev::set_is_main_thread;
use rdev::{listen, stop_listen, Button, Event, EventType};
use std::{
    fs::File,
    io::BufWriter,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self},
    time::Instant,
};
pub struct Engine {
    running: Arc<AtomicBool>,
}

impl Engine {
    pub fn start<F>(
        &self,
        last_call_time: Arc<Mutex<Instant>>,
        token_handler: F,
        writer: Arc<Mutex<BufWriter<File>>>,
    ) where
        F: 'static + Fn(Arc<Mutex<Instant>>, Vec<Token>, Arc<Mutex<BufWriter<File>>>) + Send,
    {
        self.stop();
        let running = Arc::clone(&self.running);
        thread::spawn(move || {
            running.store(true, Ordering::SeqCst);
            let callback = move |event: Event| {
                let last_call_time = Arc::clone(&last_call_time);
                let writer = Arc::clone(&writer);
                // 在mac上监听不到 Caps lock todo
                if let Ok(tokens) = event.event_type.to_token() {
                    token_handler(last_call_time, tokens, writer);
                }
            };

            #[cfg(target_os = "macos")]
            set_is_main_thread(false);

            listen(callback).unwrap();
            running.store(false, Ordering::SeqCst);
        });
    }

    pub fn stop(&self) {
        stop_listen();
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

trait EventTypeExt {
    fn to_token(&self) -> Result<Vec<input::Token>>;
}

impl EventTypeExt for EventType {
    fn to_token(&self) -> Result<Vec<input::Token>> {
        let mut tokens = Vec::new();
        match self {
            EventType::KeyPress(record_key) => {
                let key: input::Key = record_key.to_owned().try_into()?;
                let system_key: input::SystemKey = key.into();
                tokens.push(Token::Key(system_key, input::Press));
            }
            EventType::KeyRelease(record_key) => {
                let key: input::Key = record_key.to_owned().try_into()?;
                let system_key: input::SystemKey = key.into();
                tokens.push(Token::Key(system_key, input::Release));
            }
            EventType::MouseMove { x, y } => {
                let (x, y) = (x.to_owned() as i32, y.to_owned() as i32);
                tokens.push(Token::MoveMouse(x, y, input::Abs));
            }
            EventType::Wheel { delta_x, delta_y } => {
                let (x, y) = (delta_x.to_owned() as i32, delta_y.to_owned() as i32);
                if x != 0 {
                    tokens.push(Token::Scroll(x, input::Horizontal));
                }
                if y != 0 {
                    tokens.push(Token::Scroll(y, input::Vertical));
                }
            }
            EventType::ButtonPress(button) => match button {
                Button::Left => tokens.push(Token::Button(input::Button::Left, input::Press)),
                Button::Right => tokens.push(Token::Button(input::Button::Right, input::Press)),
                Button::Middle => tokens.push(Token::Button(input::Button::Middle, input::Press)),
                Button::Unknown(_) => {}
            },
            EventType::ButtonRelease(button) => match button {
                Button::Left => tokens.push(Token::Button(input::Button::Left, input::Release)),
                Button::Right => tokens.push(Token::Button(input::Button::Right, input::Release)),
                Button::Middle => tokens.push(Token::Button(input::Button::Middle, input::Release)),
                Button::Unknown(_) => {}
            },
        };
        Ok(tokens)
    }
}
