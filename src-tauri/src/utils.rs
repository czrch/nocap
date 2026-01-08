use crate::models::{FsEntry, FsEntryKind, ImageFile};
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

fn entry_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}

fn sort_fs_entries(entries: &mut [FsEntry]) {
    entries.sort_by(|a, b| {
        let kind_order = match (&a.kind, &b.kind) {
            (FsEntryKind::Directory, FsEntryKind::File) => std::cmp::Ordering::Less,
            (FsEntryKind::File, FsEntryKind::Directory) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        };

        if kind_order == std::cmp::Ordering::Equal {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        } else {
            kind_order
        }
    });
}

fn build_fs_entry(path: &Path, depth: u8) -> Result<FsEntry, String> {
    let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
    let kind = if metadata.is_dir() {
        FsEntryKind::Directory
    } else {
        FsEntryKind::File
    };

    let mut children = Vec::new();
    if matches!(kind, FsEntryKind::Directory) && depth > 0 {
        let entries = std::fs::read_dir(path).map_err(|e| e.to_string())?;
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if let Ok(child) = build_fs_entry(&entry_path, depth - 1) {
                children.push(child);
            }
        }
        sort_fs_entries(&mut children);
    }

    Ok(FsEntry {
        path: path.to_string_lossy().to_string(),
        name: entry_name(path),
        kind,
        children,
    })
}

pub fn build_fs_tree(dir_path: &Path, depth: u8) -> Result<FsEntry, String> {
    if !dir_path.exists() {
        return Err("Directory does not exist".to_string());
    }

    build_fs_entry(dir_path, depth)
}
