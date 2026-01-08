use crate::models::{FsEntry, ImageFile};
use crate::utils::{build_fs_tree, scan_directory_for_images};
use std::path::Path;

/// Get all images in the same directory as the current image path
#[tauri::command]
pub async fn get_adjacent_images(current_path: String) -> Result<Vec<ImageFile>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&current_path);

        let dir = path.parent().ok_or("Could not get parent directory")?;

        let images = scan_directory_for_images(dir);
        Ok(images)
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}

/// Scan a directory for all images
#[tauri::command]
pub async fn scan_folder_for_images(folder_path: String) -> Result<Vec<ImageFile>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&folder_path);

        if !path.exists() {
            return Err("Directory does not exist".to_string());
        }

        if !path.is_dir() {
            return Err("Path is not a directory".to_string());
        }

        let images = scan_directory_for_images(path);
        Ok(images)
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}

/// Build a directory tree up to a given depth.
#[tauri::command]
pub async fn list_dir_tree(root_path: String, depth: u8) -> Result<FsEntry, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&root_path);

        if !path.exists() {
            return Err("Directory does not exist".to_string());
        }

        if !path.is_dir() {
            return Err("Path is not a directory".to_string());
        }

        build_fs_tree(path, depth)
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}
