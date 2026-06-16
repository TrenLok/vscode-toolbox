use serde::Serialize;

#[cfg(target_os = "macos")]
use crate::vscode_uri::vscode_uri_open_arg;

#[cfg(target_os = "macos")]
use std::{
  collections::HashSet,
  path::{Path, PathBuf},
  process::Command,
  sync::OnceLock,
};

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
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

#[cfg(target_os = "macos")]
struct VSCodeProduct {
  app_name: &'static str,
  bundle_identifier: &'static str,
  channel: VSCodeChannel,
  cli_name: &'static str,
}

#[cfg(target_os = "macos")]
#[derive(Clone)]
struct VSCodeApp {
  path: PathBuf,
  bundle_identifier: &'static str,
  channel: VSCodeChannel,
  cli_name: &'static str,
}

#[cfg(target_os = "macos")]
const VSCODE_PRODUCTS: [VSCodeProduct; 3] = [
  VSCodeProduct {
    app_name: "Visual Studio Code.app",
    bundle_identifier: "com.microsoft.VSCode",
    channel: VSCodeChannel::Stable,
    cli_name: "code",
  },
  VSCodeProduct {
    app_name: "Visual Studio Code - Insiders.app",
    bundle_identifier: "com.microsoft.VSCodeInsiders",
    channel: VSCodeChannel::Insider,
    cli_name: "code-insiders",
  },
  VSCodeProduct {
    app_name: "VSCodium.app",
    bundle_identifier: "com.vscodium",
    channel: VSCodeChannel::Vscodium,
    cli_name: "codium",
  },
];

#[cfg(target_os = "macos")]
static VSCODE_APP: OnceLock<Option<VSCodeApp>> = OnceLock::new();

#[cfg(target_os = "macos")]
fn push_existing_app_candidate(
  candidates: &mut Vec<VSCodeApp>,
  product: &VSCodeProduct,
  candidate: PathBuf,
) {
  if candidate.is_dir() {
    candidates.push(VSCodeApp {
      path: candidate,
      bundle_identifier: product.bundle_identifier,
      channel: product.channel,
      cli_name: product.cli_name,
    });
  }
}

#[cfg(target_os = "macos")]
fn push_standard_app_candidates(candidates: &mut Vec<VSCodeApp>, product: &VSCodeProduct) {
  push_existing_app_candidate(
    candidates,
    product,
    PathBuf::from("/Applications").join(product.app_name),
  );

  if let Ok(home) = std::env::var("HOME") {
    push_existing_app_candidate(
      candidates,
      product,
      Path::new(&home).join("Applications").join(product.app_name),
    );
  }
}

#[cfg(target_os = "macos")]
fn push_mdfind_app_candidates(candidates: &mut Vec<VSCodeApp>, product: &VSCodeProduct) {
  let query = format!(
    "kMDItemCFBundleIdentifier == '{}'",
    product.bundle_identifier
  );
  let Ok(output) = Command::new("/usr/bin/mdfind").arg(&query).output() else {
    return;
  };

  if !output.status.success() {
    return;
  }

  for path in String::from_utf8_lossy(&output.stdout).lines() {
    let path = path.trim();
    if !path.is_empty() {
      push_existing_app_candidate(candidates, product, PathBuf::from(path));
    }
  }
}

#[cfg(target_os = "macos")]
fn vscode_app_candidates() -> Vec<VSCodeApp> {
  let mut candidates = Vec::new();

  for product in &VSCODE_PRODUCTS {
    push_standard_app_candidates(&mut candidates, product);
    push_mdfind_app_candidates(&mut candidates, product);
  }

  let mut checked = HashSet::new();
  candidates
    .into_iter()
    .filter(|candidate| checked.insert(candidate.path.clone()))
    .collect()
}

#[cfg(target_os = "macos")]
fn resolve_vscode_app() -> Option<VSCodeApp> {
  vscode_app_candidates().into_iter().next()
}

#[cfg(target_os = "macos")]
fn get_vscode_app() -> Result<&'static VSCodeApp, String> {
  VSCODE_APP
    .get_or_init(resolve_vscode_app)
    .as_ref()
    .ok_or_else(|| "Visual Studio Code.app was not found on this macOS system".into())
}

#[cfg(target_os = "macos")]
fn read_vscode_version_from_plist(app: &VSCodeApp) -> Result<String, String> {
  let plist_path = app.path.join("Contents/Info.plist");
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

#[tauri::command]
pub fn get_vscode_version_macos() -> Result<VSCodeVersion, String> {
  #[cfg(target_os = "macos")]
  {
    let app = get_vscode_app()?;
    let version = read_vscode_version_from_plist(app)?;

    Ok(VSCodeVersion {
      version,
      channel: app.channel,
    })
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
    let app = get_vscode_app()?;
    Command::new("/usr/bin/open")
      .args(["-b", app.bundle_identifier])
      .arg(&folder)
      .spawn()
      .map_err(|error| format!("Couldn't open folder in VS Code: {}", error))?;

    Ok(())
  }

  #[cfg(not(target_os = "macos"))]
  {
    let _ = folder;
    Err("This command is only available on macOS".into())
  }
}

#[tauri::command]
pub fn open_vscode_project_uri_macos(uri: String) -> Result<(), String> {
  #[cfg(target_os = "macos")]
  {
    let app = get_vscode_app()?;
    let cli_path = app
      .path
      .join("Contents")
      .join("Resources")
      .join("app")
      .join("bin")
      .join(app.cli_name);

    if !cli_path.is_file() {
      return Err(format!(
        "VS Code CLI was not found at {}",
        cli_path.display()
      ));
    }

    let open_arg = vscode_uri_open_arg(&uri);

    Command::new(&cli_path)
      .args([open_arg, &uri])
      .spawn()
      .map_err(|error| {
        format!(
          "Couldn't open remote project in VS Code using {}: {}",
          cli_path.display(),
          error
        )
      })?;

    Ok(())
  }

  #[cfg(not(target_os = "macos"))]
  {
    let _ = uri;
    Err("This command is only available on macOS".into())
  }
}
