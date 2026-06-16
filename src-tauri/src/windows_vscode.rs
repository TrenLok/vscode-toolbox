#[cfg(target_os = "windows")]
use crate::vscode_uri::vscode_uri_open_arg;
use serde::Serialize;
#[cfg(target_os = "windows")]
use std::{os::windows::process::CommandExt, path::PathBuf, process::Command, sync::OnceLock};
#[cfg(target_os = "windows")]
use winreg::{
  enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
  RegKey,
};

#[cfg(target_os = "windows")]
struct VSCodeProduct {
  app_paths_exes: &'static [&'static str],
  channel: VSCodeChannel,
  cli_name: &'static str,
  display_names: &'static [&'static str],
  exe_name: &'static str,
  install_dir: &'static str,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
enum VSCodeChannel {
  Stable,
  Insider,
  Vscodium,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VSCodeVersion {
  version: String,
  channel: VSCodeChannel,
}

#[cfg(target_os = "windows")]
const VSCODE_PRODUCTS: [VSCodeProduct; 3] = [
  VSCodeProduct {
    app_paths_exes: &["Code.exe"],
    channel: VSCodeChannel::Stable,
    cli_name: "code.cmd",
    display_names: &["Visual Studio Code"],
    exe_name: "Code.exe",
    install_dir: "Microsoft VS Code",
  },
  VSCodeProduct {
    app_paths_exes: &["code-insiders.exe", "Code - Insiders.exe"],
    channel: VSCodeChannel::Insider,
    cli_name: "code-insiders.cmd",
    display_names: &[
      "Visual Studio Code Insiders",
      "Visual Studio Code - Insiders",
    ],
    exe_name: "Code - Insiders.exe",
    install_dir: "Microsoft VS Code Insiders",
  },
  VSCodeProduct {
    app_paths_exes: &["VSCodium.exe"],
    channel: VSCodeChannel::Vscodium,
    cli_name: "codium.cmd",
    display_names: &["VSCodium"],
    exe_name: "VSCodium.exe",
    install_dir: "VSCodium",
  },
];

#[cfg(target_os = "windows")]
const VSCODE_UNINSTALL_REGISTRY_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[cfg(target_os = "windows")]
static VSCODE_APP: OnceLock<Option<VSCodeApp>> = OnceLock::new();

#[cfg(target_os = "windows")]
#[derive(Clone)]
struct VSCodeApp {
  path: PathBuf,
  channel: VSCodeChannel,
  cli_name: &'static str,
}

#[cfg(target_os = "windows")]
fn push_existing_app_candidate(
  candidates: &mut Vec<VSCodeApp>,
  product: &VSCodeProduct,
  candidate: Option<PathBuf>,
) {
  if let Some(candidate) = candidate {
    if candidate.is_file() {
      candidates.push(VSCodeApp {
        path: candidate,
        channel: product.channel,
        cli_name: product.cli_name,
      });
    }
  }
}

#[cfg(target_os = "windows")]
fn normalize_registry_path(path: String) -> PathBuf {
  let path = path.trim().trim_matches('"');
  let path = path.strip_suffix(",0").unwrap_or(path).trim_matches('"');
  PathBuf::from(path)
}

#[cfg(target_os = "windows")]
fn product_matches_display_name(product: &VSCodeProduct, display_name: &str) -> bool {
  if product.exe_name == "Code.exe" && display_name.contains("Insiders") {
    return false;
  }

  product
    .display_names
    .iter()
    .any(|name| display_name.contains(name))
}

#[cfg(target_os = "windows")]
fn push_registry_app_candidates(
  candidates: &mut Vec<VSCodeApp>,
  product: &VSCodeProduct,
  registry_roots: &[RegKey],
) {
  for app_paths_exe in product.app_paths_exes {
    let app_paths_key = format!(
      r"Software\Microsoft\Windows\CurrentVersion\App Paths\{}",
      app_paths_exe
    );

    for root in registry_roots {
      if let Ok(key) = root.open_subkey(&app_paths_key) {
        let app_exe = key.get_value::<String, _>("").map(normalize_registry_path);
        push_existing_app_candidate(candidates, product, app_exe.ok());

        let path = key
          .get_value::<String, _>("Path")
          .map(normalize_registry_path);
        push_existing_app_candidate(
          candidates,
          product,
          path.ok().map(|path| path.join(product.exe_name)),
        );
      }
    }
  }

  for root in registry_roots {
    let Ok(uninstall) = root.open_subkey(VSCODE_UNINSTALL_REGISTRY_KEY) else {
      continue;
    };

    for subkey_name in uninstall.enum_keys().filter_map(Result::ok) {
      let Ok(subkey) = uninstall.open_subkey(subkey_name) else {
        continue;
      };

      let display_name = subkey
        .get_value::<String, _>("DisplayName")
        .unwrap_or_default();
      if !product_matches_display_name(product, &display_name) {
        continue;
      }

      let install_location = subkey
        .get_value::<String, _>("InstallLocation")
        .map(normalize_registry_path);
      push_existing_app_candidate(
        candidates,
        product,
        install_location
          .ok()
          .map(|path| path.join(product.exe_name)),
      );

      let display_icon = subkey
        .get_value::<String, _>("DisplayIcon")
        .map(normalize_registry_path);
      push_existing_app_candidate(candidates, product, display_icon.ok());
    }
  }
}

#[cfg(target_os = "windows")]
fn push_standard_app_candidates(candidates: &mut Vec<VSCodeApp>, product: &VSCodeProduct) {
  if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
    push_existing_app_candidate(
      candidates,
      product,
      Some(
        PathBuf::from(&local_app_data)
          .join("Programs")
          .join(product.install_dir)
          .join(product.exe_name),
      ),
    );
  }

  for env_name in ["ProgramFiles", "ProgramFiles(x86)"] {
    if let Ok(program_files) = std::env::var(env_name) {
      push_existing_app_candidate(
        candidates,
        product,
        Some(
          PathBuf::from(&program_files)
            .join(product.install_dir)
            .join(product.exe_name),
        ),
      );
    }
  }
}

#[cfg(target_os = "windows")]
fn vscode_app_candidates() -> Vec<VSCodeApp> {
  let mut candidates = Vec::new();
  let registry_roots = [
    RegKey::predef(HKEY_CURRENT_USER),
    RegKey::predef(HKEY_LOCAL_MACHINE),
  ];

  for product in &VSCODE_PRODUCTS {
    push_registry_app_candidates(&mut candidates, product, &registry_roots);
    push_standard_app_candidates(&mut candidates, product);
  }

  candidates
}

#[cfg(target_os = "windows")]
fn resolve_vscode_app() -> Option<VSCodeApp> {
  vscode_app_candidates().into_iter().next()
}

#[cfg(target_os = "windows")]
fn vscode_app_command() -> Command {
  let mut command = if let Some(app) = VSCODE_APP.get_or_init(resolve_vscode_app).as_ref() {
    Command::new(&app.path)
  } else {
    Command::new("Code.exe")
  };

  command.creation_flags(CREATE_NO_WINDOW);
  command
}

#[cfg(target_os = "windows")]
fn vscode_cli_command() -> Result<Command, String> {
  let app = VSCODE_APP
    .get_or_init(resolve_vscode_app)
    .as_ref()
    .ok_or_else(|| {
      "Visual Studio Code executable was not found on this Windows system".to_string()
    })?;
  let cli_path = app
    .path
    .parent()
    .map(|path| path.join("bin").join(app.cli_name))
    .filter(|path| path.is_file());
  let mut command = if let Some(cli_path) = cli_path {
    Command::new(cli_path)
  } else {
    Command::new(app.cli_name)
  };

  command.creation_flags(CREATE_NO_WINDOW);
  Ok(command)
}

#[cfg(target_os = "windows")]
fn vscode_package_json_path() -> Option<PathBuf> {
  let app = VSCODE_APP.get_or_init(resolve_vscode_app).as_ref()?;
  let app_dir = app.path.parent()?;
  let direct_path = app_dir.join("resources").join("app").join("package.json");
  if direct_path.is_file() {
    return Some(direct_path);
  }

  let entries = std::fs::read_dir(app_dir).ok()?;
  let mut nested_paths = entries
    .filter_map(Result::ok)
    .map(|entry| {
      entry
        .path()
        .join("resources")
        .join("app")
        .join("package.json")
    })
    .filter(|candidate| candidate.is_file())
    .collect::<Vec<_>>();
  nested_paths.sort();

  if let Some(candidate) = nested_paths.into_iter().next() {
    return Some(candidate);
  }

  Some(direct_path)
}

#[cfg(target_os = "windows")]
fn read_vscode_version_from_package_json() -> Result<VSCodeVersion, String> {
  let channel = VSCODE_APP
    .get_or_init(resolve_vscode_app)
    .as_ref()
    .map(|app| app.channel)
    .ok_or_else(|| {
      "Visual Studio Code executable was not found on this Windows system".to_string()
    })?;
  let package_json_path = vscode_package_json_path().ok_or_else(|| {
    "Visual Studio Code executable was not found on this Windows system".to_string()
  })?;
  let package_json = std::fs::read_to_string(&package_json_path).map_err(|error| {
    format!(
      "Couldn't read VS Code package metadata from {}: {}",
      package_json_path.display(),
      error
    )
  })?;
  let package_json: serde_json::Value = serde_json::from_str(&package_json).map_err(|error| {
    format!(
      "Couldn't parse VS Code package metadata from {}: {}",
      package_json_path.display(),
      error
    )
  })?;
  let version = package_json
    .get("version")
    .and_then(serde_json::Value::as_str)
    .ok_or_else(|| {
      format!(
        "VS Code version is missing in {}",
        package_json_path.display()
      )
    })?;

  Ok(VSCodeVersion {
    version: version.to_string(),
    channel,
  })
}

#[cfg(target_os = "windows")]
fn read_vscode_version_from_command() -> Result<VSCodeVersion, String> {
  let app = VSCODE_APP
    .get_or_init(resolve_vscode_app)
    .as_ref()
    .ok_or_else(|| {
      "Visual Studio Code executable was not found on this Windows system".to_string()
    })?;
  let output = vscode_cli_command()?.arg("-v").output().map_err(|error| {
    format!(
      "Couldn't read VS Code version from {}: {}",
      app.cli_name, error
    )
  })?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    return Err(format!(
      "Couldn't read VS Code version from {}{}",
      app.cli_name,
      if stderr.is_empty() {
        String::new()
      } else {
        format!(": {}", stderr)
      }
    ));
  }

  let version = String::from_utf8_lossy(&output.stdout)
    .lines()
    .next()
    .map(str::trim)
    .unwrap_or_default()
    .to_string();
  if version.is_empty() {
    return Err(format!("VS Code version is empty from {}", app.cli_name));
  }

  Ok(VSCodeVersion {
    version,
    channel: app.channel,
  })
}

#[tauri::command]
pub fn get_vscode_version_windows() -> Result<VSCodeVersion, String> {
  #[cfg(target_os = "windows")]
  {
    read_vscode_version_from_package_json().or_else(|package_error| {
      read_vscode_version_from_command().map_err(|command_error| {
        format!(
          "{}; fallback to executable version check failed: {}",
          package_error, command_error
        )
      })
    })
  }

  #[cfg(not(target_os = "windows"))]
  {
    Err("This command is only available on Windows".into())
  }
}

#[tauri::command]
pub fn open_vscode_project_windows(folder: String) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    vscode_app_command()
      .args(["--", &folder])
      .spawn()
      .map_err(|error| format!("Couldn't open folder in VS Code: {}", error))?;

    Ok(())
  }

  #[cfg(not(target_os = "windows"))]
  {
    let _ = folder;
    Err("This command is only available on Windows".into())
  }
}

#[tauri::command]
pub fn open_vscode_project_uri_windows(uri: String) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    let open_arg = vscode_uri_open_arg(&uri);

    vscode_app_command()
      .args([open_arg, &uri])
      .spawn()
      .map_err(|error| format!("Couldn't open remote project in VS Code: {}", error))?;

    Ok(())
  }

  #[cfg(not(target_os = "windows"))]
  {
    let _ = uri;
    Err("This command is only available on Windows".into())
  }
}
