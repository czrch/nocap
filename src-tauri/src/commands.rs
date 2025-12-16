use std::path::Path;
use crate::models::{ImageFile, ImageMetadata};
use crate::utils::{scan_directory_for_images, extract_image_info};

/// Open a native file picker dialog for selecting a single image
#[tauri::command]
pub async fn open_image_dialog() -> Result<Option<ImageFile>, String> {
    use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
    
    let file = tauri::async_runtime::spawn(async {
        FileDialogBuilder::new()
            .add_filter("Images", &["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"])
            .pick_file()
            .await
    })
    .await
    .map_err(|e| format!("Dialog error: {}", e))?;

    if let Some(path) = file {
        let path_buf = path.path;
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
pub async fn open_folder_dialog() -> Result<Vec<ImageFile>, String> {
    use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
    
    let folder = tauri::async_runtime::spawn(async {
        FileDialogBuilder::new()
            .pick_folder()
            .await
    })
    .await
    .map_err(|e| format!("Dialog error: {}", e))?;

    if let Some(path) = folder {
        let path_buf = path.path;
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
    
    let dir = path.parent()
        .ok_or("Could not get parent directory")?;
    
    let images = scan_directory_for_images(dir);
    Ok(images)
}

/// Extract metadata from an image file
#[tauri::command]
pub fn get_image_metadata(path: String) -> Result<ImageMetadata, String> {
    let path_ref = Path::new(&path);
    extract_image_info(path_ref)
}
