use fs_extra::dir;
use pyproject::PyProject;
use simple_zip::{compress, extract};
use std::{env, fs, path::PathBuf, process::Command, sync::OnceLock};

fn main() {
    init();
    // init_7z();
    // init_vscode();
    init_python_interpreter();

    if is_macos() {
        init_attr();
    }

    init_whl();

    if is_macos() {
        tauri_build_macos();
    }

    if is_windows() {
        tauri_build_windows();
    }
}

fn tauri_build_macos() {
    tauri_build::build()
}

fn tauri_build_windows() {
    let windows = tauri_build::WindowsAttributes::new().app_manifest(
        r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
        type="win32"
        name="Microsoft.Windows.Common-Controls"
        version="6.0.0.0"
        processorArchitecture="*"
        publicKeyToken="6595b64144ccf1df"
        language="*"
      />
    </dependentAssembly>
  </dependency>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#,
    );
    tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
        .expect("Failed to run build script.");
}

fn init_attr() {
    let output = Command::new(&"xattr")
        .args(&[
            "-r",
            "-d",
            "com.apple.quarantine",
            target_python_interpreter().to_str().unwrap(),
        ])
        .output()
        .expect("Failed to xattr target_python_interpreter.");
    if !output.status.success() {
        println!("cargo:warning=init_attr xattr target_python_interpreter failed");
    }
}

fn init_whl() {
    let wheels_dir = assets_dir().join("python").join("wheels");
    let kiwi_package_dir = assets_dir().join("python").join("packages").join("kiwi");
    let kiwi_whl_path = assets_dir()
        .join("pytho")
        .join("wheels")
        .join(&format!("kiwi-{}-py3-none-any.whl", kiwi_whl_version()));

    if assets_dir()
        .join("python")
        .join("wheels")
        .join(&format!("kiwi-{}-py3-none-any.whl", kiwi_whl_version()))
        .exists()
    {
        return;
    }

    println!("cargo:warning=upgrade target python module: pip");
    let output = command_python()
        .args(&["-u", "-m", "pip", "install", "--upgrade", "pip"])
        .output()
        .expect("Failed to upgrade pip");

    if !output.status.success() {
        println!("cargo:warning=init_whl upgarde pip failed");
    }

    println!("cargo:warning=install target python module: build");
    let output = command_python()
        .args(&["-u", "-m", "pip", "install", "build"])
        .output()
        .expect("Failed to install build");

    if !output.status.success() {
        println!("cargo:warning=init_whl install build failed");
    }

    println!("cargo:warning=build python module: kiwi");
    let output = command_python()
        .args(&[
            "-u",
            "-m",
            "build",
            "-o",
            wheels_dir.to_str().unwrap(),
            kiwi_package_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to build kiwi whl");

    if !output.status.success() {
        println!("cargo:warning=init_whl build kiwi whl failed");
    }

    println!("cargo:warning=download dependencies of python module kiwi");
    let output = command_python()
        .args(&[
            "-u",
            "-m",
            "pip",
            "download",
            kiwi_whl_path.to_str().unwrap(),
            "-d",
            wheels_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to download dependencies of kiwi whl");

    if !output.status.success() {
        println!("cargo:warning=init_whl download dependencies of kiwi whl failed");
    }
}

fn init_python_interpreter() {
    let dst_file = assets_dir().join("zip").join("interpreter.zip");

    if dst_file.exists() {
        return;
    }

    println!("cargo:warning=compress python interpreter");
    let interpreter_name = {
        if is_macos() {
            "python_interpreter_macos"
        } else {
            "python_interpreter_windows"
        }
    };
    let src_dir = assets_dir().join("resources").join(interpreter_name);
    compress(&src_dir, &dst_file).expect("Compress python interpreter failed.");
    {
        println!("cargo:warning=extract python interpreter to target");
        let src_path = dst_file;
        let dst_dir = target_dir().join("python").join("interpreter");
        dir::create_all(&dst_dir, true).expect("Failed to create target/python/interpreter dir.");
        extract(fs::File::open(src_path).unwrap(), &dst_dir, true)
            .expect("Failed to extract python interpreter.");
    }
}

#[allow(dead_code)]
fn init_vscode() {
    let dst_file_one = assets_dir().join("zip").join("vscode_windows.zip.001");

    if dst_file_one.exists() {
        return;
    }

    let src_dir = assets_dir().join("resources").join("vscode_windows");
    let dst_file = assets_dir().join("zip").join("vscode_windows.zip");
    let mut command = command_7z();
    println!("cargo:warning=compress vscode_windows via 7z");
    command
        .args(&[
            "a",
            "-tzip",
            "-v80m",
            dst_file.to_str().unwrap(),
            src_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Compress vscode_windows failed.");
}

#[allow(dead_code)]
fn init_7z() {
    let dst_file = assets_dir().join("zip").join("7z.zip");

    if dst_file.exists() {
        return;
    }

    println!("cargo:warning=compress 7z");
    let src_dir = assets_dir().join("resources").join("7z");
    compress(&src_dir, &dst_file).expect("Compress 7z failed.");
}

fn init() {
    if !((cfg!(target_os = "macos") && cfg!(target_arch = "aarch64"))
        || (cfg!(target_os = "windows")
            && !cfg!(target_arch = "arm")
            && !cfg!(target_arch = "aarch64")))
    {
        panic!("unsupported platform");
    }

    PROFILE.get_or_init(|| env::var("PROFILE").unwrap().to_string());
    IS_DEV.get_or_init(|| env::var("PROFILE").unwrap() == "debug".to_string());
    IS_MACOS.get_or_init(|| cfg!(target_os = "macos"));
    IS_WINDOWS.get_or_init(|| cfg!(target_os = "windows"));
}

fn target_dir() -> PathBuf {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    PathBuf::from(&cargo_manifest_dir)
        .join("target")
        .join(&profile)
}

fn assets_dir() -> PathBuf {
    PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets")
}

fn kiwi_whl_version() -> String {
    PyProject::default().project.version
}

fn is_macos() -> bool {
    IS_MACOS.get().unwrap().clone()
}

fn is_windows() -> bool {
    IS_WINDOWS.get().unwrap().clone()
}

fn command_7z() -> Command {
    if is_macos() {
        Command::new("7z")
    } else {
        Command::new(&assets_dir().join("resources").join("7z").join("7za.exe"))
    }
}

fn target_python_interpreter() -> PathBuf {
    let path = {
        if is_macos() {
            target_dir()
                .join("python")
                .join("interpreter")
                .join("bin")
                .join("python3.13")
        } else {
            target_dir()
                .join("python")
                .join("interpreter")
                .join("python.exe")
        }
    };
    path
}

fn command_python() -> Command {
    let path = target_python_interpreter();

    if !path.exists() {
        panic!("{} is not exist.", path.to_str().unwrap());
    }

    Command::new(&path)
}

static PROFILE: OnceLock<String> = OnceLock::new();
static IS_DEV: OnceLock<bool> = OnceLock::new();
static IS_MACOS: OnceLock<bool> = OnceLock::new();
static IS_WINDOWS: OnceLock<bool> = OnceLock::new();

mod pyproject {
    use anyhow::{anyhow, Result};
    use serde::Deserialize;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct PyProject {
        #[serde(rename = "build-system")]
        build_system: BuildSystem,

        pub project: Project,

        tool: Tool,
    }
    impl Default for PyProject {
        fn default() -> Self {
            let base_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
            let toml_path = base_dir
                .join("assets")
                .join("python")
                .join("packages")
                .join("kiwi")
                .join("pyproject.toml");
            let toml_content = fs::read_to_string(&toml_path)
                .unwrap()
                .trim_start_matches('\u{feff}')
                .to_string();
            let pyproject = PyProject::load_from_toml_content(toml_content)
                .expect("load kiwi project toml failed.");
            pyproject
        }
    }

    impl PyProject {
        pub fn load_from_toml_content(toml_content: String) -> Result<PyProject> {
            let config: PyProject =
                toml::from_str(&toml_content).map_err(|error| anyhow!(error.to_string()))?;
            Ok(config)
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct BuildSystem {
        requires: Vec<String>,
        #[serde(rename = "build-backend")]
        build_backend: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct Project {
        name: String,
        pub version: String,
        description: String,
        authors: Vec<Author>,
        readme: String,
        #[serde(rename = "requires-python")]
        requires_python: String,
        dependencies: Vec<String>,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Author {
        name: String,
        email: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Tool {
        setuptools: ToolSetuptools,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct ToolSetuptools {
        packages: Vec<String>,
    }
}
