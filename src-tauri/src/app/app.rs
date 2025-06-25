use super::Config;
use crate::{
    capture::{Engine as CaptureEngine, Frame},
    input::Engine as InputEngine,
    project::Project,
    record::Engine as RecordEngine,
    APP_VERSION,
};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tauri::AppHandle;

pub struct App {
    project: Arc<RwLock<Option<Project>>>,
    config: Arc<RwLock<Config>>,
}

impl App {
    fn init_capturer() -> &'static Arc<CaptureEngine> {
        CAPTURER.get_or_init(|| Arc::new(CaptureEngine::default()))
    }

    fn init_recorder() -> &'static Arc<RecordEngine> {
        RECORDER.get_or_init(|| Arc::new(RecordEngine::default()))
    }

    fn init_input() -> &'static Arc<InputEngine> {
        INPUT.get_or_init(|| Arc::new(InputEngine::default()))
    }

    pub fn get() -> Arc<Self> {
        APP.get_or_init(|| Arc::new(App::default())).clone()
    }

    pub fn get_frame() -> Result<Frame> {
        Self::with_capturer(|capturer| capturer.get_frame().ok_or(anyhow!("Frame is not found.")))
    }

    pub fn get_frame_arc() -> Result<Arc<Frame>> {
        Self::with_capturer(|capturer| {
            capturer
                .get_frame_arc()
                .ok_or(anyhow!("Frame is not found."))
        })
    }

    pub fn capturer() -> Arc<CaptureEngine> {
        Self::init_capturer().clone()
    }

    pub fn recorder() -> Arc<RecordEngine> {
        Self::init_recorder().clone()
    }

    pub fn input() -> Arc<InputEngine> {
        Self::init_input().clone()
    }

    pub fn project(&self) -> Result<RwLockReadGuard<Option<Project>>> {
        self.project
            .read()
            .map_err(|e| anyhow!("Failed to acquire project read lock: {}", e))
    }

    pub fn project_mut(&self) -> Result<RwLockWriteGuard<Option<Project>>> {
        self.project
            .write()
            .map_err(|e| anyhow!("Failed to acquire project write lock: {}", e))
    }

    pub fn config(&self) -> Result<RwLockReadGuard<Config>> {
        self.config
            .read()
            .map_err(|e| anyhow!("Failed to acquire config read lock: {}", e))
    }

    pub fn config_mut(&self) -> Result<RwLockWriteGuard<Config>> {
        self.config
            .write()
            .map_err(|e| anyhow!("Failed to acquire config write lock: {}", e))
    }

    pub fn with_capturer<R>(f: impl FnOnce(&CaptureEngine) -> R) -> R {
        f(&*Self::init_capturer())
    }

    pub fn with_recorder<R>(f: impl FnOnce(&RecordEngine) -> R) -> R {
        f(&*Self::init_recorder())
    }

    pub fn with_input<R>(f: impl FnOnce(&InputEngine) -> R) -> R {
        f(&*Self::init_input())
    }

    pub fn try_with_project<R>(f: impl FnOnce(&Project) -> R) -> Result<R> {
        let app = App::get();
        let guard = app.project.read().unwrap();

        if let Some(project) = guard.as_ref() {
            Ok(f(project))
        } else {
            Err(anyhow!("Project not found"))
        }
    }

    pub fn try_with_project_mut<R>(f: impl FnOnce(&mut Project) -> R) -> Result<R> {
        let app = App::get();
        let mut guard = app.project.write().unwrap();

        if let Some(project) = guard.as_mut() {
            Ok(f(project))
        } else {
            Err(anyhow!("Project not found"))
        }
    }

    pub fn with_config<R>(f: impl FnOnce(&Config) -> R) -> R {
        let app = App::get();
        let guard = app.config.read().unwrap();
        f(&*guard)
    }

    pub fn with_config_mut<R>(f: impl FnOnce(&mut Config) -> R) -> R {
        let app = App::get();
        let mut guard = app.config.write().unwrap();
        f(&mut *guard)
    }

    pub fn get_resource_dir() -> &'static PathBuf {
        RESOURCE_DIR.get().unwrap()
    }

    pub fn init_resource_dir(resource_dir: PathBuf) -> Result<()> {
        RESOURCE_DIR
            .set(resource_dir)
            .map_err(|_| anyhow!("Failed to init resource dir"))?;
        Ok(())
    }

    pub fn init_app_handle(app_handle: Arc<AppHandle>) -> Result<()> {
        APP_HANDLE
            .set(app_handle)
            .map_err(|_| anyhow!("Failed to init app handle"))?;
        Ok(())
    }

    pub fn get_app_handle() -> Arc<AppHandle> {
        APP_HANDLE.get().unwrap().clone()
    }

    pub fn get_version() -> &'static str {
        APP_VERSION
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            project: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(Config::default())),
        }
    }
}

static APP: OnceLock<Arc<App>> = OnceLock::new();
static CAPTURER: OnceLock<Arc<CaptureEngine>> = OnceLock::new();
static RECORDER: OnceLock<Arc<RecordEngine>> = OnceLock::new();
static INPUT: OnceLock<Arc<InputEngine>> = OnceLock::new();
static RESOURCE_DIR: OnceLock<PathBuf> = OnceLock::new();
static APP_HANDLE: OnceLock<Arc<AppHandle>> = OnceLock::new();
