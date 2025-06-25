use super::{PyProject, PythonCode};
use crate::{app::App, project::VerifyStatus};
use anyhow::{anyhow, Result};
use fs_extra;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{ChildStderr, ChildStdout, Command, ExitStatus, Stdio},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

#[cfg(windows)]
use std::os::windows::process::CommandExt as _;
#[cfg(windows)]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

#[derive(Clone)]
pub struct Engine {
    pub default_interpreter: PathBuf,
    pub project_interpreter: PathBuf,
    pub project_path: PathBuf,
    pub pid: Arc<AtomicU32>,
}

impl Engine {
    pub fn new_from_project_path(project_path: impl AsRef<Path>) -> Self {
        let default_interpreter = get_default_interpreter();
        let project_path = project_path.as_ref().to_path_buf();
        Self {
            default_interpreter,
            project_interpreter: get_project_interpreter(&project_path),
            project_path,
            pid: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn verify_in_project(project_path: impl AsRef<Path>) -> VerifyStatus {
        let project_path = PathBuf::from(project_path.as_ref());
        // Check if main.py exists or VerifyStatus::Invalid
        {
            let path = project_path.join("main.py");
            if let Err(_) = fs::exists(&path).map_err(|error| anyhow!(error)) {
                return VerifyStatus::Invalid;
            }
        }
        // Check if interpreter exists or VerifyStatus::Invalid
        {
            let path = get_project_interpreter(&project_path);
            if let Err(_) = fs::exists(&path).map_err(|error| anyhow!(error)) {
                return VerifyStatus::Invalid;
            }
        }
        // Check if data/images exists or VerifyStatus::Invalid
        {
            let path = project_path.join("data").join("images");
            if let Err(_) = fs::exists(&path).map_err(|error| anyhow!(error)) {
                return VerifyStatus::Invalid;
            }
        }
        {
            let path = project_path.join(".venv").join("pyvenv.cfg");
            if let Err(_) = fs::exists(&path).map_err(|error| anyhow!(error)) {
                return VerifyStatus::Invalid;
            }
            let content = {
                match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(_) => return VerifyStatus::Invalid,
                }
            };
            // Check if project moved or VerifyStatus::Moved
            {
                let venv_path = project_path.join(".venv");
                let not_moved = content.lines().any(|line| {
                    if let Some(cmd_line) = line.trim().strip_prefix("command = ") {
                        if let Some(index) = cmd_line.find("-m venv ") {
                            let venv_str = cmd_line[index + "-m venv ".len()..].trim();
                            return Path::new(venv_str) == venv_path.as_path();
                        }
                    }
                    false
                });
                if !not_moved {
                    return VerifyStatus::Moved;
                }
            }
            // Check if kiwi whl installed or VerifyStatus::Invalid
            {
                let interpreter = get_project_interpreter(&project_path);
                let output = {
                    let result = {
                        #[cfg(target_os = "macos")]
                        {
                            Command::new(&interpreter)
                                .args(&["-u", "-m", "pip", "show", "kiwi"])
                                .output()
                        }
                        #[cfg(target_os = "windows")]
                        {
                            Command::new(&interpreter)
                                .args(&["-u", "-m", "pip", "show", "kiwi"])
                                .creation_flags(CREATE_NO_WINDOW.0)
                                .output()
                        }
                    };
                    match result {
                        Ok(o) => o,
                        Err(_) => return VerifyStatus::Invalid,
                    }
                };
                let stdout = String::from_utf8_lossy(&output.stdout);
                let installed = output.status.success() && stdout.contains("Name: kiwi");

                if !installed {
                    return VerifyStatus::Invalid;
                }
            }
        }
        VerifyStatus::Valid
    }

    pub fn init(&self) -> Result<()> {
        let default_interpreter = self.default_interpreter.clone();
        //copy template to project
        {
            let options = fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true);
            let template_dir = App::get_resource_dir()
                .join("python")
                .join("project_template");
            fs_extra::dir::copy(&template_dir, &self.project_path, &options)
                .map_err(|error| anyhow!("Copy template to project failed.({})", error))?;
        }
        // rename .vscode/settings.json
        {
            let vscode_path_buf = self.project_path.join(".vscode");
            let dst = vscode_path_buf.join("settings.json");

            #[cfg(target_os = "macos")]
            let src = vscode_path_buf.join("settings.json.mac");

            #[cfg(target_os = "windows")]
            let src = vscode_path_buf.join("settings.json.windows");

            fs::rename(&src, &dst)
                .map_err(|error| anyhow!("Rename .vscode/settings.json failed.({})", error))?;
        }
        //venv
        {
            let venv_name = ".venv";
            let venv_path = self.project_path.join(venv_name);
            let result = {
                #[cfg(target_os = "macos")]
                {
                    Command::new(&default_interpreter)
                        .args(&["-u", "-m", "venv"])
                        .arg(&venv_path)
                        .output()
                }
                #[cfg(target_os = "windows")]
                {
                    Command::new(&default_interpreter)
                        .args(&["-u", "-m", "venv"])
                        .arg(&venv_path)
                        .creation_flags(CREATE_NO_WINDOW.0)
                        .output()
                }
            };

            match result {
                Ok(output) => {
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(anyhow!("Command stderr: {}", stderr));
                    }
                }
                Err(error) => {
                    return Err(anyhow!("Command execution error: {}", error));
                }
            }
        }
        // install kiwi whl
        {
            let wheels_path_buf = get_wheels_path();
            {
                let pyproject = PyProject::default();
                let version = pyproject.project.version;
                let whl = format!("kiwi-{}-py3-none-any.whl", version);
                let kiwi = wheels_path_buf.join(&whl);
                let find_links = format!("--find-links={}", &wheels_path_buf.to_str().unwrap());
                let result = {
                    #[cfg(target_os = "macos")]
                    {
                        Command::new(&self.project_interpreter)
                            .args(&["-u", "-m", "pip", "install", "--no-index"])
                            .arg(&find_links)
                            .arg(&kiwi)
                            .output()
                    }
                    #[cfg(target_os = "windows")]
                    {
                        Command::new(&self.project_interpreter)
                            .args(&["-u", "-m", "pip", "install", "--no-index"])
                            .arg(&find_links)
                            .arg(&kiwi)
                            .creation_flags(CREATE_NO_WINDOW.0)
                            .output()
                    }
                };

                match result {
                    Ok(output) => {
                        if !output.status.success() {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            return Err(anyhow!("Command stderr: {}", stderr));
                        }
                    }
                    Err(error) => {
                        return Err(anyhow!("Command execution error: {}", error));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn run<
        OnSpawned: Fn(u32) + Send + 'static,
        OnStdout: Fn(ChildStdout) + Send + 'static,
        OnStderr: Fn(ChildStderr) + Send + 'static,
        OnExit: Fn(u32, ExitStatus) + Send + 'static,
    >(
        &self,
        path: impl AsRef<Path>,
        port: u16,
        on_spawned: OnSpawned,
        on_stdout: OnStdout,
        on_stderr: OnStderr,
        on_exit: OnExit,
    ) -> Result<()> {
        let child = {
            #[cfg(target_os = "macos")]
            {
                Command::new(&self.project_interpreter)
                    .arg("-u")
                    .arg(path.as_ref())
                    .arg("--port")
                    .arg(port.to_string())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
            }
            #[cfg(target_os = "windows")]
            {
                Command::new(&self.project_interpreter)
                    .arg("-u")
                    .arg(path.as_ref())
                    .arg("--port")
                    .arg(port.to_string())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .creation_flags(CREATE_NO_WINDOW.0)
            }
        }
        .spawn();
        let mut child = child.map_err(|error| {
            return anyhow!("unwrap project python interpreter error.({})", error);
        })?;
        let pid = child.id();
        on_spawned(pid);
        self.pid.store(pid, Ordering::SeqCst);
        if let Some(stdout) = child.stdout.take() {
            on_stdout(stdout);
        }
        if let Some(stderr) = child.stderr.take() {
            on_stderr(stderr);
        }
        match child.wait() {
            Ok(exit_status) => on_exit(pid, exit_status),
            Err(error) => {
                return Err(anyhow!("child try wait error.({})", error));
            }
        }
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    pub fn stop(&self) -> bool {
        let pid = self.pid.load(Ordering::SeqCst);
        if pid == 0 {
            return false;
        }
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
        self.pid.store(0, Ordering::SeqCst);
        true
    }

    pub fn get_pid(&self) -> u32 {
        self.pid.load(Ordering::SeqCst)
    }
}

impl Engine {
    pub fn code(&self) -> PythonCode {
        PythonCode::default()
    }
}

fn get_project_interpreter(project_path: impl AsRef<Path>) -> PathBuf {
    let project_path = project_path.as_ref();

    #[cfg(target_os = "macos")]
    {
        project_path.join(".venv").join("bin").join("python")
    }

    #[cfg(target_os = "windows")]
    {
        project_path
            .join(".venv")
            .join("Scripts")
            .join("python.exe")
    }
}

pub fn get_default_interpreter() -> PathBuf {
    let base_path = App::get_resource_dir().join("python").join("interpreter");

    #[cfg(target_os = "macos")]
    {
        base_path.join("bin").join("python3.13")
    }

    #[cfg(target_os = "windows")]
    {
        base_path.join("python.exe")
    }
}

fn get_wheels_path() -> PathBuf {
    App::get_resource_dir().join("python").join("wheels")
}
