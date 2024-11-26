use crate::{
    common::{PROJECTS_DIR, PYTHON_EXEC_FILE},
    utils::current_time,
};

pub mod capture;
pub mod find;
pub mod fs;
pub mod install;

use install::TESSERACT_DIR;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn projects_dir() -> String {
    PROJECTS_DIR.to_string()
}

#[tauri::command]
pub fn has_permission() -> bool {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::Shell::IsUserAnAdmin;
        unsafe { IsUserAnAdmin().as_bool() }
    }
    #[cfg(target_os = "macos")]
    {
        true
    }
}

#[tauri::command]
pub fn init() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        //add tesseract to PATH
        let current_path = env::var("PATH").unwrap_or_else(|_| String::new());
        let new_path = format!("{};{:?}", current_path, TESSERACT_DIR.to_string());
        env::set_var("PATH", new_path);
        //allow python can output chinese
        env::set_var("PYTHONUTF8", "1");
        env::set_var("PYTHONIOENCODING", "utf-8");
        Ok(true)
    }
    #[cfg(target_os = "macos")]
    {
        Ok(true)
    }
}

#[tauri::command]
pub fn run(app: AppHandle, file: String) {
    let app: Arc<Mutex<AppHandle>> = Arc::new(Mutex::new(app));
    if *PID.lock().unwrap() != 0 {
        app.lock().unwrap().emit_with_timestamp(
            "log:info",
            format!("script is already running (pid: {}).", *PID.lock().unwrap()).as_str(),
        );
        return;
    }
    std::thread::spawn(move || {
        let handle = Command::new(PYTHON_EXEC_FILE.to_string())
            .arg(file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        if let Err(error) = handle {
            app.lock()
                .unwrap()
                .emit_with_timestamp("log:error", &error.to_string());
            return;
        }
        let mut handle = handle.unwrap();
        let pid = handle.id();
        *PID.lock().unwrap() = pid;
        app.lock().unwrap().emit_with_timestamp(
            "log:info",
            format!("script is started (pid: {}).", pid).as_str(),
        );
        let app_stdout = Arc::clone(&app);
        let app_stderr = Arc::clone(&app);
        let mut stdout_thread = std::thread::spawn(move || {});
        let mut stderr_thread = std::thread::spawn(move || {});
        if let Some(stdout) = handle.stdout.take() {
            stdout_thread = std::thread::spawn(move || {
                let app = Arc::clone(&app_stdout);
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        app.lock().unwrap().emit_with_timestamp("log:info", &line);
                    }
                }
            });
        }
        if let Some(stderr) = handle.stderr.take() {
            stderr_thread = std::thread::spawn(move || {
                let app = Arc::clone(&app_stderr);
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        app.lock().unwrap().emit_with_timestamp("log:info", &line);
                    }
                }
            });
        }
        match handle.wait() {
            Ok(_) => {
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
            }
            Err(error) => {
                app.lock()
                    .unwrap()
                    .emit_with_timestamp("log:error", &error.to_string());
            }
        };
        *PID.lock().unwrap() = 0;
    });
}

#[tauri::command]
pub fn stop(app: AppHandle) {
    let pid = *PID.lock().unwrap();
    if pid == 0 {
        app.emit_with_timestamp("log:info", "pid does not exist");
        return;
    }
    *PID.lock().unwrap() = 0;
    #[cfg(target_os = "windows")]
    {
        println!("kill pid: {}", pid);
        let handle = Command::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(pid.to_string())
            .spawn();
        if let Ok(mut handle) = handle {
            let _ = handle.wait();
        }
    }
    #[cfg(target_os = "macos")]
    {
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        }
    }
    app.emit_with_timestamp(
        "log:info",
        format!("script is stopped (pid: {}).", pid).as_str(),
    );
}

lazy_static! {
    pub static ref PID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

trait AppHandleExt {
    fn emit_with_timestamp(&self, target: &str, data: &str);
}

impl AppHandleExt for AppHandle {
    fn emit_with_timestamp(&self, target: &str, data: &str) {
        let time = current_time();
        let payload = Emit::new(data.to_string(), time);
        self.emit(target, payload).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Emit {
    data: String,
    time: f64,
}
impl Emit {
    fn new(data: String, time: f64) -> Self {
        Self { data, time }
    }
}
