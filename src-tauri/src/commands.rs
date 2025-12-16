use crate::models::{ImageFile, ImageMetadata};
use crate::utils::{extract_image_info, scan_directory_for_images};
use std::path::Path;
use tauri_plugin_dialog::DialogExt;

/// Open a native file picker dialog for selecting a single image
#[tauri::command]
pub fn open_image_dialog(app: tauri::AppHandle) -> Result<Option<ImageFile>, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    app.dialog()
        .file()
        .add_filter(
            "Images",
            &["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"],
        )
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });

    let file = rx.recv().map_err(|e| e.to_string())?;

    if let Some(file_path) = file {
        let path_buf = std::path::PathBuf::from(file_path.to_string());
        if let (Some(filename), Some(extension)) = (
            path_buf.file_name().and_then(|n| n.to_str()),
            path_buf.extension().and_then(|e| e.to_str()),
        ) {
            Ok(Some(ImageFile {
                path: path_buf.to_string_lossy().to_string(),
                filename: filename.to_string(),
                extension: extension.to_lowercase(),
            }))
        } else {
            Err("Invalid file path".to_string())
        }
    } else {
        Ok(None)
    }
}

/// Open a native folder picker dialog and return all images in that folder
#[tauri::command]
pub fn open_folder_dialog(app: tauri::AppHandle) -> Result<Vec<ImageFile>, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    app.dialog().file().pick_folder(move |folder_path| {
        let _ = tx.send(folder_path);
    });

    let folder = rx.recv().map_err(|e| e.to_string())?;

    if let Some(folder_path) = folder {
        let path_buf = std::path::PathBuf::from(folder_path.to_string());
        let images = scan_directory_for_images(&path_buf);
        Ok(images)
    } else {
        Ok(Vec::new())
    }
}

/// Get all images in the same directory as the current image path
#[tauri::command]
pub fn get_adjacent_images(current_path: String) -> Result<Vec<ImageFile>, String> {
    let path = Path::new(&current_path);

    let dir = path.parent().ok_or("Could not get parent directory")?;

    let images = scan_directory_for_images(dir);
    Ok(images)
}

/// Scan a directory for all images
#[tauri::command]
pub fn scan_folder_for_images(folder_path: String) -> Result<Vec<ImageFile>, String> {
    let path = Path::new(&folder_path);

    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let images = scan_directory_for_images(path);
    Ok(images)
}

/// Extract metadata from an image file
#[tauri::command]
pub fn get_image_metadata(path: String) -> Result<ImageMetadata, String> {
    let path_ref = Path::new(&path);
    extract_image_info(path_ref)
}
