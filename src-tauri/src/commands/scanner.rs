use crate::db::open_project_db;
use crate::models::{MediaFile, MediaType};
use rusqlite::Error as SqlError;
use std::path::Path;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::fs;
use std::io::{self, Read};
use hex;
use chrono::{DateTime, Utc, TimeDelta};

#[tauri::command]
pub fn scan_directory(project_path: String) -> Result<Vec<MediaFile>, String> {
  let path = Path::new(&project_path);
  let conn = open_project_db(path).map_err(|e| e.to_string())?;

  let mut media_files = Vec::new();

  for entry in WalkDir::new(path).follow_links(false).max_depth(10).into_iter().filter_map(|e| e.ok()) {
    if entry.file_type().is_file() {
      let full_path = entry.path();
      if let Some(extension) = full_path.extension().and_then(|s| s.to_str()) {
        let ext_lower = extension.to_lowercase();
        let media_type = match ext_lower.as_str() {
          "jpg" | "jpeg" | "png" | "gif" | "webp" | "heic" => Some(MediaType::Image),
          "mp4" | "mov" | "avi" | "mkv" => Some(MediaType::Video),
          "mp3" | "flac" | "wav" | "m4a" => Some(MediaType::Audio),
          _ => None,
        };
        if let Some(mt) = media_type {
          let filename = full_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
          let metadata = fs::metadata(full_path).map_err(|e| e.to_string())?;
          let file_size = Some(metadata.len() as i64);
          let modified = metadata.modified().ok().map(|t| {
            let secs = t.duration_since(std::time::UNIX_EPOCH).map_err(|_| "invalid time".to_string())?.as_secs();
            DateTime::<Utc>::from_timestamp(secs as i64, TimeDelta::zero()).unwrap().to_rfc3339()
          });
          let created_at = None;
          let file_hash = compute_file_hash(full_path).ok();

          let tx = conn.transaction().map_err(|e| e.to_string())?;

          tx.execute(
            "INSERT OR IGNORE INTO media_files (path, filename, extension, media_type, file_size, file_hash, modified_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[&full_path.to_string_lossy().to_string(), &filename, &extension.to_string(), &mt.to_string(), &file_size, &file_hash.as_deref(), &modified.as_deref()],
          ).map_err(|e| e.to_string())?;

          tx.commit().map_err(|e| e.to_string())?;

          let media_file: MediaFile = conn.query_row(
            "SELECT id, path, filename, extension, media_type, file_size, created_at, modified_at, thumbnail_path FROM media_files WHERE path = ?1",
            [&full_path.to_string_lossy().to_string()],
            |row| {
              Ok(MediaFile {
                id: row.get(0)?,
                path: row.get(1)?,
                filename: row.get(2)?,
                extension: row.get(3)?,
                media_type: row.get(4)?,
                file_size: row.get(5)?,
                created_at: row.get(6)?,
                modified_at: row.get(7)?,
                thumbnail_path: row.get(8)?,
              })
            },
          ).map_err(|e| e.to_string())?;

          media_files.push(media_file);
        }
      }
    }
  }

  Ok(media_files)
}

fn compute_file_hash(path: &Path) -> Result<String, io::Error> {
  let mut file = fs::File::open(path)?;
  let mut hasher = Sha256::new();
  let mut buffer = [0u8; 4096];
  loop {
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }
    hasher.update(&buffer[..bytes_read]);
  }
  Ok(hex::encode(hasher.finalize()))
}