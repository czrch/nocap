use crate::models::{ExifSummary, ImageFile, ImageMetadata};
use exif::{In, Tag};
use std::fs::File;
use std::io::BufReader;
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

pub fn image_file_from_path(path: &Path) -> Option<ImageFile> {
    let filename = path.file_name()?.to_str()?.to_string();
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_lowercase();

    Some(ImageFile {
        path: path.to_string_lossy().to_string(),
        filename,
        extension,
    })
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

fn is_supported_exif_container(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "tif" | "tiff"))
}

fn exif_field_string(exif: &exif::Exif, tag: Tag) -> Option<String> {
    exif.get_field(tag, In::PRIMARY)
        .map(|field| field.display_value().with_unit(exif).to_string())
}

fn exif_orientation_string(exif: &exif::Exif) -> Option<String> {
    let field = exif
        .get_field(Tag::Orientation, In::PRIMARY)
        .or_else(|| exif.get_field(Tag::Orientation, In::THUMBNAIL))?;

    let value = field.value.get_uint(0)?;
    Some(match value {
        1 => "normal".to_string(),
        2 => "mirrored".to_string(),
        3 => "rotated 180°".to_string(),
        4 => "mirrored upside-down".to_string(),
        5 => "mirrored, rotated 90°".to_string(),
        6 => "rotated 90°".to_string(),
        7 => "mirrored, rotated 270°".to_string(),
        8 => "rotated 270°".to_string(),
        other => format!("{other}"),
    })
}

/// Extract a small EXIF summary from an image file.
///
/// Returns `Ok(None)` when the file doesn't support EXIF (by extension) or no EXIF is found.
pub fn extract_exif_summary(path: &Path) -> Result<Option<ExifSummary>, String> {
    if !is_supported_exif_container(path) {
        return Ok(None);
    }

    let file = File::open(path).map_err(|e| format!("Failed to open file for EXIF: {}", e))?;
    let mut bufreader = BufReader::new(&file);

    let exif = match exif::Reader::new().read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        Err(exif::Error::Io(e)) => {
            return Err(format!("Failed to read EXIF: {}", e));
        }
        Err(_) => {
            // Missing or malformed EXIF is treated as "no EXIF" for a minimal viewer.
            return Ok(None);
        }
    };

    let date_taken = exif_field_string(&exif, Tag::DateTimeOriginal)
        .or_else(|| exif_field_string(&exif, Tag::DateTimeDigitized))
        .or_else(|| exif_field_string(&exif, Tag::DateTime));

    Ok(Some(ExifSummary {
        date_taken,
        camera_make: exif_field_string(&exif, Tag::Make),
        camera_model: exif_field_string(&exif, Tag::Model),
        lens_model: exif_field_string(&exif, Tag::LensModel),
        author: exif_field_string(&exif, Tag::Artist),
        description: exif_field_string(&exif, Tag::ImageDescription),
        copyright: exif_field_string(&exif, Tag::Copyright),
        software: exif_field_string(&exif, Tag::Software),
        orientation: exif_orientation_string(&exif),
    }))
}
