use crate::models::ImageFile;
use std::path::Path;

/// Check if a file path has a supported image extension
pub fn is_supported_image(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_str.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg"
        )
    } else {
        false
    }
}

/// Scan a directory for all supported image files
pub fn scan_directory_for_images(dir_path: &Path) -> Vec<ImageFile> {
    let mut images = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && is_supported_image(&path) {
                if let (Some(filename), Some(extension)) = (
                    path.file_name().and_then(|n| n.to_str()),
                    path.extension().and_then(|e| e.to_str()),
                ) {
                    images.push(ImageFile {
                        path: path.to_string_lossy().to_string(),
                        filename: filename.to_string(),
                        extension: extension.to_lowercase(),
                    });
                }
            }
        }
    }

    // Sort by filename for consistent ordering
    images.sort_by(|a, b| a.filename.cmp(&b.filename));
    images
}
