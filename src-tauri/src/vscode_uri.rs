pub fn vscode_uri_open_arg(uri: &str) -> &'static str {
  let path = uri.split(['?', '#']).next().unwrap_or(uri);

  if path.to_lowercase().ends_with(".code-workspace") {
    "--file-uri"
  } else {
    "--folder-uri"
  }
}
