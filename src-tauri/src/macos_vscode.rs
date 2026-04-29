#[cfg(target_os = "macos")]
use std::{
  path::{Path, PathBuf},
  process::Command,
};

#[cfg(target_os = "macos")]
const VSCODE_BUNDLE_IDENTIFIER: &str = "com.microsoft.VSCode";

#[cfg(target_os = "macos")]
fn resolve_vscode_app_path() -> Result<PathBuf, String> {
  let mut candidates = vec![PathBuf::from("/Applications/Visual Studio Code.app")];

  if let Ok(home) = std::env::var("HOME") {
    candidates.push(Path::new(&home).join("Applications/Visual Studio Code.app"));
  }

  let query = format!("kMDItemCFBundleIdentifier == '{VSCODE_BUNDLE_IDENTIFIER}'");
  if let Ok(output) = Command::new("/usr/bin/mdfind").arg(&query).output() {
    if output.status.success() {
      for path in String::from_utf8_lossy(&output.stdout).lines() {
        let path = path.trim();
        if !path.is_empty() {
          candidates.push(PathBuf::from(path));
        }
      }
    }
  }

  let mut checked = std::collections::HashSet::new();

  for candidate in candidates {
    if !checked.insert(candidate.clone()) {
      continue;
    }

    if candidate.is_dir() {
      return Ok(candidate);
    }
  }

  Err("Visual Studio Code.app was not found on this macOS system".into())
}

#[tauri::command]
pub fn get_vscode_version_macos() -> Result<String, String> {
  #[cfg(target_os = "macos")]
  {
    let app_path = resolve_vscode_app_path()?;
    let plist_path = app_path.join("Contents/Info.plist");
    let output = Command::new("/usr/bin/plutil")
      .args(["-extract", "CFBundleShortVersionString", "raw"])
      .arg(&plist_path)
      .output()
      .map_err(|error| {
        format!(
          "Couldn't read VS Code version from {}: {}",
          plist_path.display(),
          error
        )
      })?;

    if !output.status.success() {
      let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
      return Err(format!(
        "Couldn't extract VS Code version from {}{}",
        plist_path.display(),
        if stderr.is_empty() {
          String::new()
        } else {
          format!(": {}", stderr)
        }
      ));
    }

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if version.is_empty() {
      return Err(format!(
        "VS Code version is empty in {}",
        plist_path.display()
      ));
    }

    Ok(version)
  }

  #[cfg(not(target_os = "macos"))]
  {
    Err("This command is only available on macOS".into())
  }
}

#[tauri::command]
pub fn open_vscode_project_macos(folder: String) -> Result<(), String> {
  #[cfg(target_os = "macos")]
  {
    let _ = resolve_vscode_app_path()?;

    let status = Command::new("/usr/bin/open")
      .args(["-b", VSCODE_BUNDLE_IDENTIFIER])
      .arg(&folder)
      .status()
      .map_err(|error| format!("Couldn't open folder in VS Code: {}", error))?;

    if status.success() {
      Ok(())
    } else {
      Err(format!(
        "VS Code failed to open folder, exit status: {}",
        status
      ))
    }
  }

  #[cfg(not(target_os = "macos"))]
  {
    let _ = folder;
    Err("This command is only available on macOS".into())
  }
}
