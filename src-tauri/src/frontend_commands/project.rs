use super::AppHandleExt as _;
use crate::{
    capture::{listen_primary_display, CAPTURE_SWITCH},
    common::{PROJECT_DIR, PYTHON_EXEC_FILE},
};
use lazy_static::lazy_static;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};
use tauri::AppHandle;

#[cfg(target_os = "windows")]
use install::TESSERACT_DIR;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;
/// project about
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
    app.lock()
        .unwrap()
        .emit_with_timestamp("run:status", "running");
    {
        *CAPTURE_SWITCH.lock().unwrap() = true;
        listen_primary_display();
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
                app.lock()
                    .unwrap()
                    .emit_with_timestamp("run:status", "stopped");
            }
            Err(error) => {
                app.lock()
                    .unwrap()
                    .emit_with_timestamp("log:error", &error.to_string());
            }
        };
        {
            *CAPTURE_SWITCH.lock().unwrap() = false;
            *PID.lock().unwrap() = 0;
        }
    });
}

#[tauri::command]
pub fn stop(app: AppHandle) {
    {
        *CAPTURE_SWITCH.lock().unwrap() = false;
    }
    let pid = *PID.lock().unwrap();
    if pid == 0 {
        app.emit_with_timestamp("log:info", "pid does not exist");
        return;
    }
    *PID.lock().unwrap() = 0;
    #[cfg(target_os = "windows")]
    {
        let handle = Command::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(pid.to_string())
            .creation_flags(CREATE_NO_WINDOW.0)
            .spawn();
        if let Ok(mut handle) = handle {
            let _ = handle.wait();
        }
    }
    #[cfg(target_os = "macos")]
    {
        unsafe {
            libc::kill(pid as i32, libc::SIGKILL);
        }
    }
    app.emit_with_timestamp(
        "log:info",
        format!("script is stopped (pid: {}).", pid).as_str(),
    );
    app.emit_with_timestamp("run:status", "stopped");
}

#[tauri::command]
pub fn set_project(path: String) {
    let path_buff = PathBuf::from(path);
    *PROJECT_DIR.lock().unwrap() = Some(path_buff);
}

#[tauri::command]
pub fn get_project_dir() -> Result<String, String> {
    let project_path = PROJECT_DIR.lock().unwrap();
    match project_path.as_ref() {
        Some(path_buff) => Ok(path_buff.to_str().unwrap().to_string()),
        None => Ok("".to_string()),
    }
}

lazy_static! {
    pub static ref PID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}
