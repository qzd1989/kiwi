use super::frame::Frame;
use crate::app::App;
use crate::types::Size;
use anyhow::{anyhow, Result};
use arc_swap::ArcSwapOption;
use capture::{Config, Engine as CaptureEngine, Frame as CaptureFrame};
use display_info::DisplayInfo;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

#[cfg(target_os = "macos")]
use objc2::msg_send;
#[cfg(target_os = "macos")]
use objc2_foundation::NSObject;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowDisplayAffinity, WINDOW_DISPLAY_AFFINITY};

pub struct Engine {
    engine: Arc<CaptureEngine>,
    frame: ArcSwapOption<Frame>,
}

impl Engine {
    pub fn start(&self) -> Result<()> {
        self.engine.start()
    }

    pub fn start_background(&self) -> Result<()> {
        self.engine.start_background()
    }

    pub fn stop(&self) {
        self.engine.stop();
    }

    pub fn is_running(&self) -> bool {
        self.engine.is_running()
    }

    pub fn grab(&self, time_out_millis: u64) -> Result<Frame> {
        let frame: Frame = if self.is_running() {
            self.get_frame()
                .ok_or_else(|| anyhow!("Capturer is running, but frame is none."))?
        } else {
            self.engine.grab(time_out_millis)?.into()
        };
        Ok(frame)
    }

    pub fn get_monitor_size(&self) -> Result<Size> {
        let infos = DisplayInfo::all().map_err(|error| anyhow!(error))?;
        let Some(primary) = infos.into_iter().find(|info| info.is_primary) else {
            return Err(anyhow!("No primary display found"));
        };
        let size = Size {
            width: primary.width,
            height: primary.height,
        };
        Ok(size)
    }

    // 返回cloned,不影响原数据
    pub fn get_frame(&self) -> Option<Frame> {
        self.frame.load_full().as_ref().map(|arc| (**arc).clone())
    }

    // 返回arc,更快,只读
    pub fn get_frame_arc(&self) -> Option<Arc<Frame>> {
        self.frame.load_full()
    }

    pub fn clear_frame(&self) {
        self.frame.store(None);
    }
}

impl Engine {
    pub fn hide_windows_from_capture(app_handle: &AppHandle, windows: &[String]) -> Result<()> {
        for window in windows {
            Self::hide_window_from_capture(&app_handle, window)?;
        }
        Ok(())
    }

    pub fn show_windows_from_capture(app_handle: &AppHandle, windows: &[String]) -> Result<()> {
        for window in windows {
            Self::show_window_from_capture(&app_handle, window)?;
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn hide_window_from_capture(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle
            .get_webview_window(label)
            .ok_or_else(|| anyhow!("Failed to get webview by label: {}", label))?;
        match window.hwnd() {
            Ok(hwnd) => unsafe {
                SetWindowDisplayAffinity(hwnd, WINDOW_DISPLAY_AFFINITY(17))
                    .map_err(|error| anyhow!(error))
            },
            Err(error) => Err(anyhow!(error)),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn show_window_from_capture(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle
            .get_webview_window(label)
            .ok_or_else(|| anyhow!("Failed to get webview by label: {}", label))?;
        match window.hwnd() {
            Ok(hwnd) => unsafe {
                SetWindowDisplayAffinity(hwnd, WINDOW_DISPLAY_AFFINITY(0))
                    .map_err(|error| anyhow!(error))
            },
            Err(error) => Err(anyhow!(error)),
        }
    }

    #[cfg(target_os = "macos")]
    fn hide_window_from_capture(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle
            .get_webview_window(label)
            .ok_or_else(|| anyhow!("Failed to get webview window: {}", label))?;
        let ns_window = window.ns_window()?;
        unsafe {
            let ns_object = &*(ns_window as *const NSObject);
            let _: () = msg_send![ns_object, setSharingType: 0u32];
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub fn show_window_from_capture(app_handle: &AppHandle, label: &str) -> Result<()> {
        let window = app_handle
            .get_webview_window(label)
            .ok_or_else(|| anyhow!("Failed to get webview window: {}", label))?;
        let ns_window = window.ns_window()?;

        unsafe {
            let ns_object = &*(ns_window as *const NSObject);
            let _: () = msg_send![ns_object, setSharingType: 1u32];
        }

        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        let config = Config::new(capture::Format::RGBA);
        let on_frame_arrived = Box::new(|frame: CaptureFrame| {
            let new_frame = Frame::new(frame.width, frame.height, frame.buffer);
            App::with_capturer(|capturer| {
                capturer.frame.store(Some(Arc::new(new_frame)));
            });
        });
        let engine = CaptureEngine::new(config, on_frame_arrived);
        Self {
            engine,
            frame: ArcSwapOption::empty(),
        }
    }
}
