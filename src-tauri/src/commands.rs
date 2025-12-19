use crate::models::{ExifSummary, ImageFile, ImageMetadata};
use crate::utils::{
    extract_exif_summary, extract_image_info, image_file_from_path, scan_directory_for_images,
};
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

/// Extract metadata from an image file
#[tauri::command]
pub async fn get_image_metadata(path: String) -> Result<ImageMetadata, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path_ref = Path::new(&path);
        extract_image_info(path_ref)
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}

/// Extract EXIF metadata summary from an image file (when present)
#[tauri::command]
pub async fn get_exif_summary(path: String) -> Result<Option<ExifSummary>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path_ref = Path::new(&path);
        extract_exif_summary(path_ref)
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}

/// Rename the current image file (same directory).
///
/// This is used for "Save" when the user edits the filename.
#[tauri::command]
pub async fn rename_image_file(path: String, new_name: String) -> Result<ImageFile, String> {
    tauri::async_runtime::spawn_blocking(move || {
        if new_name.trim().is_empty() {
            return Err("New name cannot be empty".to_string());
        }

        if new_name.contains('/') || new_name.contains('\\') {
            return Err("New name must not include path separators".to_string());
        }

        let src = Path::new(&path);
        if !src.exists() {
            return Err("File does not exist".to_string());
        }
        if !src.is_file() {
            return Err("Path is not a file".to_string());
        }

        let dir = src.parent().ok_or("Could not get parent directory")?;

        let mut final_name = new_name.trim().to_string();
        let has_ext = Path::new(&final_name).extension().is_some();
        if !has_ext {
            if let Some(old_ext) = src.extension().and_then(|e| e.to_str()) {
                if !old_ext.is_empty() {
                    final_name.push('.');
                    final_name.push_str(old_ext);
                }
            }
        }

        let dest = dir.join(final_name);
        if dest.exists() && dest != src {
            return Err("A file with that name already exists".to_string());
        }

        std::fs::rename(src, &dest).map_err(|e| format!("Failed to rename file: {}", e))?;

        image_file_from_path(&dest).ok_or("Failed to build file info".to_string())
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}

/// Create a copy of the current image file at the given destination path.
///
/// This is used for "Save As".
#[tauri::command]
pub async fn save_as_copy(path: String, destination_path: String) -> Result<ImageFile, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let src = Path::new(&path);
        if !src.exists() {
            return Err("File does not exist".to_string());
        }
        if !src.is_file() {
            return Err("Path is not a file".to_string());
        }

        let dest = Path::new(&destination_path);
        let dest_parent = dest.parent().ok_or("Could not get destination directory")?;
        if !dest_parent.exists() {
            return Err("Destination directory does not exist".to_string());
        }

        if dest.exists() {
            return Err("Destination file already exists".to_string());
        }

        std::fs::copy(src, dest).map_err(|e| format!("Failed to copy file: {}", e))?;

        image_file_from_path(dest).ok_or("Failed to build file info".to_string())
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}
