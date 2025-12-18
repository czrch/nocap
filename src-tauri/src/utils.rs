use crate::models::{ImageFile, ImageMetadata};
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

/// Extract image metadata using the image crate
pub fn extract_image_info(path: &Path) -> Result<ImageMetadata, String> {
    // Get file size
    let metadata =
        std::fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let size = metadata.len();

    // SVG isn't supported by the `image` crate; treat it as display-only and return basic metadata.
    if path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"))
    {
        return Ok(ImageMetadata {
            path: path.to_string_lossy().to_string(),
            width: 0,
            height: 0,
            size,
            format: "svg".to_string(),
        });
    }

    // Get image dimensions and format (header-only; avoids decoding full image)
    let img_reader = image::ImageReader::open(path)
        .and_then(|r| r.with_guessed_format())
        .map_err(|e| format!("Failed to open image: {}", e))?;

    let format = img_reader
        .format()
        .ok_or("Could not determine image format")?;

    let (width, height) = img_reader
        .into_dimensions()
        .map_err(|e| format!("Failed to read image dimensions: {}", e))?;

    Ok(ImageMetadata {
        path: path.to_string_lossy().to_string(),
        width,
        height,
        size,
        format: format!("{:?}", format).to_lowercase(),
    })
}
