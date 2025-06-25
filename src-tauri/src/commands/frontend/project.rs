use super::{
    error::CommandResult,
    utils::{emit, emit_progress, get_relative_image_data_path_buf},
};
use crate::app::{App, Log};
use crate::extensions::AppHandleExt as _;
use crate::{
    interpreter::Interpreter,
    project::{Config, Project, ProjectInfo, VerifyStatusString},
    types::{Base64Png, Base64PngExt as _, Progress, Size},
};
use anyhow::Error;
use fs_extra::dir;
use std::{
    fs::File,
    io::{BufRead as _, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tauri::{ipc::Response, AppHandle};

#[tauri::command]
pub fn save_project(name: String, language: String, path: String) -> CommandResult<()> {
    let project_path = PathBuf::from(path);

    if let Err(error) = dir::create_all(&project_path, false) {
        return Err(format!(
            "Failed to create project directory: {:?}, error is: {}",
            &project_path, error
        )
        .into());
    }

    let interpreter = Interpreter::new_from_language_and_project_path(&language, &project_path)?;
    let config = Config::new(
        &name,
        &language,
        &Project::get_edit_command(&interpreter),
        &Project::get_kiwi_version(&interpreter),
    );
    config.save(&project_path)?;

    Ok(())
}

#[tauri::command]
pub fn init_project(app_handle: AppHandle, path: String) -> CommandResult<()> {
    let project_path = PathBuf::from(path);
    let config = Config::new_from_toml(&project_path)?;
    let interpreter =
        Interpreter::new_from_language_and_project_path(&config.project.language, &project_path)?;
    thread::spawn(move || {
        emit_progress(&app_handle, "init_project", Progress::start());

        if let Err(error) = interpreter.init() {
            Log::error(error.to_string()).send_to_app_log();
            return;
        }

        emit_progress(&app_handle, "init_project", Progress::finished());
    });
    Ok(())
}

#[tauri::command]
pub fn reinit_project(app_handle: AppHandle, path: String) -> CommandResult<()> {
    use fs_extra::remove_items;
    let project_path = PathBuf::from(path);
    let config = Config::new_from_toml(&project_path)?;
    let interpreter =
        Interpreter::new_from_language_and_project_path(&config.project.language, &project_path)?;
    thread::spawn(move || {
        emit_progress(&app_handle, "reinit_project", Progress::start());

        //delete
        {
            let mut need_to_removed = Vec::new();
            let venv_path = project_path.join(".venv");
            need_to_removed.push(&venv_path);

            if let Err(error) = remove_items(&need_to_removed) {
                Log::error(error.to_string()).send_to_app_log();
                return;
            }
        }

        //init
        if let Err(error) = interpreter.init() {
            Log::error(error.to_string()).send_to_app_log();
            return;
        }

        emit_progress(&app_handle, "reinit_project", Progress::finished());
    });
    Ok(())
}

#[tauri::command]
pub fn verify_project(path: String) -> CommandResult<VerifyStatusString> {
    let project_path = PathBuf::from(path);
    Ok(Project::verify(&project_path).to_string())
}

#[tauri::command]
pub fn open_project(app: AppHandle, path: String) -> CommandResult<ProjectInfo> {
    let app_handle = Arc::new(app);
    let project_path = PathBuf::from(path);
    let project = Project::new_from_project_path(project_path)?;
    let project_info: ProjectInfo = ProjectInfo::from(&project);
    let app = App::get();
    let mut project_guard = app.project_mut()?;
    *project_guard = Some(project);
    emit(&app_handle, "backend:update:project", &project_info);
    Ok(project_info)
}

#[tauri::command]
pub fn reveal_project_folder() -> CommandResult<()> {
    App::try_with_project(|project| project.reveal_folder()).map_err(|error| error.into())
}

#[tauri::command]
pub fn open_project_in_editor() -> CommandResult<()> {
    App::try_with_project(|project| project.open_in_editor())
        .map_err(|error| error.to_string())??;
    Ok(())
}

#[tauri::command]
pub fn get_project() -> CommandResult<ProjectInfo> {
    let project_info = App::try_with_project(|project| ProjectInfo::from(project))?;
    Ok(project_info)
}

#[tauri::command]
pub fn save_image(name: String, data: Base64Png) -> CommandResult<()> {
    use fs_extra::dir;
    let name = name + ".png";
    let buffer = data.to_buffer().unwrap();
    let project_path = App::try_with_project(|project| project.path.clone())?;
    let data_path = get_relative_image_data_path_buf();
    let path = Path::new(&project_path).join(&data_path).join(&name);
    let parent_path = path.parent().unwrap();
    if let Err(error) = dir::create_all(parent_path, false) {
        return Err(format!(
            "create folder failed: {}, {}",
            parent_path.to_string_lossy(),
            error
        )
        .into());
    }
    match buffer.save(path) {
        Ok(_) => Ok(()),
        Err(error) => return Err(error.into()),
    }
}

#[tauri::command]
pub fn get_image(name: String, data_path: String) -> CommandResult<Response> {
    let name = name + ".png";
    let project_path = App::try_with_project(|project| project.path.clone())?;
    let path = Path::new(&project_path).join(data_path).join(&name);
    let Ok(image) = image::open(&path) else {
        return Err(format!("can't open image: {}", path.to_string_lossy()).into());
    };
    Ok(Response::new(image.to_rgba8().to_vec()))
}

#[tauri::command]
pub fn get_image_size(name: String, data_path: String) -> CommandResult<Size> {
    let name = name + ".png";
    let project_path = App::try_with_project(|project| project.path.clone())?;
    let path = Path::new(&project_path).join(data_path).join(&name);
    let Ok(image) = image::open(&path) else {
        return Err(format!("can't open image: {}", path.to_string_lossy()).into());
    };
    Ok(Size::new(image.width(), image.height()))
}

#[tauri::command]
pub fn run(app_handle: AppHandle, path: String) {
    let app_handle = Arc::new(app_handle);

    if let Err(error) = App::try_with_project(|_| {}) {
        Log::error(error.to_string()).send_to_app_log();
        return;
    }

    if App::with_capturer(|capturer| capturer.is_running()) {
        Log::error("Screen capture is still running, please wait a moment.").send_to_app_log();
        return;
    }

    if App::with_recorder(|recorder| recorder.is_running()) {
        Log::error("The recorder is running, please stop it first.").send_to_app_log();
        return;
    }

    let app_handle_spawned = Arc::clone(&app_handle);
    let app_handle_wait = Arc::clone(&app_handle);
    let on_spawned = move |pid| {
        Log::success(format!("The script is now running, pid: {}.", pid)).send_to_app_log();
        app_handle_spawned.emit_with_timestamp("run:status", "running");
    };
    let on_stdout = move |stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    Log::info(&line).send_to_app_log();
                }
            }
        });
    };
    let on_stderr = move |stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    Log::error(&line).send_to_app_log();
                }
            }
        });
    };
    let on_exit = move |pid, exit_status| {
        Log::success(format!(
            "The script has finished running, pid: {}, {}.",
            pid, exit_status
        ))
        .send_to_app_log();
        app_handle_wait.emit_with_timestamp("run:status", "stopped");
    };

    if let Err(error) = App::with_capturer(|capturer| {
        capturer.clear_frame();
        capturer.start_background()
    }) {
        Log::error(error.to_string()).send_to_app_log();
        return;
    }

    thread::spawn(move || {
        loop {
            if App::get_frame_arc().is_ok() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        let port = App::with_config(|config| config.app.websocket_port);
        let _ = App::try_with_project(|project| {
            if let Err(error) = project
                .interpreter
                .run(path, port, on_spawned, on_stdout, on_stderr, on_exit)
            {
                Log::error(error.to_string()).send_to_app_log();
            }
        });

        App::with_capturer(|capturer| capturer.stop());
    });
}

#[tauri::command]
pub fn stop_all(app: AppHandle) {
    let app_handle = Arc::new(app);

    if let Err(error) = App::try_with_project(|_| {}) {
        Log::error(error.to_string()).send_to_app_log();
        return;
    }

    if App::with_recorder(|recorder| recorder.is_running()) {
        App::with_recorder(|recorder| {
            recorder.stop();
            Log::success("The recorder has stopped.").send_to_app_log();
            app_handle.emit_with_timestamp("run:status", "stopped");
        });
        return;
    }

    App::with_capturer(|capturer| {
        if capturer.is_running() {
            capturer.stop();
        }
    });

    let _ = App::try_with_project(|project| {
        if project.interpreter.get_pid() != 0 {
            project.interpreter.stop();
        } else {
            Log::error("No script is currently running.").send_to_app_log();
        }
    });

    app_handle.emit_with_timestamp("run:status", "stopped");
}

#[tauri::command]
pub fn run_recorder(app: AppHandle) {
    let app_handle = Arc::new(app);

    if let Err(error) = App::try_with_project(|_| {}) {
        Log::error(error.to_string()).send_to_app_log();
        return;
    }

    if App::with_capturer(|capturer| capturer.is_running()) {
        Log::error("Screen capture is still running, please wait a moment.").send_to_app_log();
        return;
    }

    if App::with_recorder(|recorder| recorder.is_running()) {
        Log::error("The recorder is running, please stop it first.").send_to_app_log();
        return;
    }

    Log::success("The recorder is now running.").send_to_app_log();

    let last_call_time = Arc::new(Mutex::new(Instant::now()));
    let file = match App::try_with_project(|project| project.generate_record_file()) {
        Ok(Ok(f)) => f,
        Err(error) | Ok(Err(error)) => {
            Log::error(error.to_string()).send_to_app_log();
            return;
        }
    };
    let file_name = match file.file_name().and_then(|f| f.to_str()) {
        Some(name) => name,
        None => {
            Log::error("Invalid file name.").send_to_app_log();
            return;
        }
    };

    app_handle.emit_with_timestamp("update:record_file", file_name);

    let file = match File::create(&file) {
        Ok(f) => f,
        Err(error) => {
            Log::error(error.to_string()).send_to_app_log();
            return;
        }
    };
    let writer = BufWriter::new(file);
    let writer = Arc::new(Mutex::new(writer));

    //write header
    {
        let template = match App::try_with_project(|project| project.get_recorder_header_template())
        {
            Ok(template) => template,
            Err(error) => {
                Log::error(error.to_string()).send_to_app_log();
                return;
            }
        };
        let mut writer = writer.lock().unwrap();

        if let Err(error) = writeln!(writer, "{}", template) {
            Log::error(error.to_string()).send_to_app_log();
        }
    }

    //write token line
    {
        let error_handler = |error: Error| {
            Log::error(format!("recorder error: {}", error.to_string())).send_to_app_log();
        };
        let token_handler = match App::try_with_project(|project| {
            project.get_recorder_token_handler(Box::new(error_handler))
        }) {
            Ok(f) => f,
            Err(error) => {
                Log::error(format!("recorder error: {}", error.to_string())).send_to_app_log();
                return;
            }
        };

        App::with_recorder(|recorder| {
            recorder.start(last_call_time, token_handler, writer);
        });
    }
}
