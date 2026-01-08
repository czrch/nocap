use crate::models::{FsChangeEvent, FsEntry, ImageFile};
use crate::state::FsWatcher;
use crate::utils::{build_fs_tree, scan_directory_for_images};
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use tauri::Emitter;

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

/// Start watching a directory recursively for changes.
#[tauri::command]
pub async fn start_watch(
    root_path: String,
    app: tauri::AppHandle,
    state: tauri::State<'_, FsWatcher>,
) -> Result<(), String> {
    let path = Path::new(&root_path).to_path_buf();

    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let app_handle = app.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            let paths = event
                .paths
                .into_iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect();
            let payload = FsChangeEvent { paths };
            let _ = app_handle.emit("fs://changed", payload);
        }
    })
    .map_err(|e| e.to_string())?;

    watcher
        .watch(&path, RecursiveMode::Recursive)
        .map_err(|e| e.to_string())?;

    let mut guard = state
        .watcher
        .lock()
        .map_err(|_| "Watcher lock poisoned".to_string())?;
    *guard = Some(watcher);

    Ok(())
}
