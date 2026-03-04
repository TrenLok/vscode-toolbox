use rusqlite::{Connection, OpenFlags};

#[tauri::command]
pub fn get_vscode_recent_from_state(db_path: String) -> Result<String, String> {
  let conn = Connection::open_with_flags(
    db_path,
    OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
  )
  .map_err(|e| e.to_string())?;

  let mut stmt = conn
    .prepare("SELECT value FROM ItemTable WHERE key = 'history.recentlyOpenedPathsList'")
    .map_err(|e| e.to_string())?;

  let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
  if let Some(row) = rows.next().map_err(|e| e.to_string())? {
    let value: String = row.get(0).map_err(|e| e.to_string())?;
    Ok(value)
  } else {
    Err("Key not found".into())
  }
}
