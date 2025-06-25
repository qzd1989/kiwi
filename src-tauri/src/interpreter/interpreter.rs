use super::python::{Engine as PythonEngine, PyProject};
use super::Code;
use crate::project::VerifyStatus;
use crate::{
    input::{SystemKey, Token},
    interpreter::python::TokenExt as _,
};
use anyhow::{anyhow, Error, Result};
use std::{
    fmt::Display,
    path::Path,
    process::{ChildStderr, ChildStdout, ExitStatus},
};
use std::{
    fs::File,
    io::{BufWriter, Write as _},
    sync::{Arc, Mutex},
    time::Instant,
};

#[derive(Clone)]
pub enum Interpreter {
    Python(PythonEngine),
    // Ruby(RubyEngine),
    // Javascript(JavascriptEngine),
}

impl Interpreter {
    pub fn new_from_language_and_project_path(
        language: impl AsRef<str>,
        project_path: impl AsRef<Path>,
    ) -> Result<Self> {
        match language.as_ref().to_lowercase().as_str() {
            "python" => Ok(Self::Python(PythonEngine::new_from_project_path(
                project_path,
            ))),
            _ => Err(anyhow!("Unsupported interpreter")),
        }
    }

    pub fn verify_in_project(
        language: impl AsRef<str>,
        project_path: impl AsRef<Path>,
    ) -> VerifyStatus {
        match language.as_ref().to_lowercase().as_str() {
            "python" => PythonEngine::verify_in_project(project_path),
            _ => VerifyStatus::Invalid,
        }
    }
}

impl Interpreter {
    pub fn init(&self) -> Result<()> {
        match self {
            Interpreter::Python(engine) => engine.init(),
        }
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
        match self {
            Interpreter::Python(engine) => {
                engine.run(path, port, on_spawned, on_stdout, on_stderr, on_exit)
            }
        }
    }

    pub fn stop(&self) -> bool {
        match self {
            Interpreter::Python(engine) => engine.stop(),
        }
    }

    pub fn get_pid(&self) -> u32 {
        match self {
            Interpreter::Python(engine) => engine.get_pid(),
        }
    }
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Interpreter::Python(_) => "python",
        };
        write!(f, "{}", name)
    }
}

impl Interpreter {
    pub fn get_code(&self) -> Box<dyn Code> {
        match self {
            Interpreter::Python(engine) => Box::new(engine.code()),
        }
    }

    pub fn get_suffix(&self) -> String {
        match self {
            Interpreter::Python(_) => "py",
        }
        .to_string()
    }

    pub fn get_main_file(&self) -> String {
        match self {
            Interpreter::Python(_) => "main.py",
        }
        .to_string()
    }

    pub fn get_kiwi_version(&self) -> String {
        match self {
            Interpreter::Python(_) => {
                let pyproject = PyProject::default();
                pyproject.project.version
            }
        }
        .to_string()
    }

    pub fn get_recorder_header_template(&self) -> String {
        match self {
            Interpreter::Python(_) => String::from(
                "from kiwi import ScreenClient, System, Key, Point\n\
             client = ScreenClient()",
            ),
        }
    }

    pub fn get_recorder_token_handler(
        &self,
        error_handler: Box<dyn Fn(Error) + Send + Sync + 'static>,
    ) -> Box<dyn Fn(Arc<Mutex<Instant>>, Vec<Token>, Arc<Mutex<BufWriter<File>>>) + Send> {
        match self {
            Interpreter::Python(_) => {
                let callback =
                    move |last_call_time: Arc<Mutex<Instant>>,
                          tokens: Vec<Token>,
                          writer: Arc<Mutex<BufWriter<File>>>| {
                        if tokens
                            .iter()
                            .any(|token| matches!(token, Token::Key(SystemKey::F12, _)))
                        {
                            return;
                        }

                        {
                            let mut last_time = last_call_time.lock().unwrap();
                            let now = Instant::now();
                            let delta = now.duration_since(*last_time);
                            let millis: u32 =
                                (delta.as_secs() * 1000 + delta.subsec_millis() as u64) as u32;
                            *last_time = now;
                            let python_script = format!("System.sleep(milliseconds={})", millis);
                            writeln!(writer.lock().unwrap(), "{}", &python_script).unwrap();
                            writer.lock().unwrap().flush().unwrap();
                        }
                        {
                            for token in tokens.iter() {
                                match token.to_python_statement() {
                                    Ok(python_script) => {
                                        let full_python_script =
                                            format!("client.{}", &python_script);
                                        writeln!(writer.lock().unwrap(), "{}", &full_python_script)
                                            .unwrap();
                                        writer.lock().unwrap().flush().unwrap();
                                    }
                                    Err(error) => {
                                        error_handler(error);
                                    }
                                }
                            }
                        }
                    };
                return Box::new(callback);
            }
        }
    }
}

impl Into<String> for Interpreter {
    fn into(self) -> String {
        match self {
            Interpreter::Python(_) => "python",
        }
        .to_string()
    }
}
