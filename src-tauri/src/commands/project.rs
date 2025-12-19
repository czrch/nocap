use crate::db::open_project_db;
use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub exists: bool,
}

/// Open a project directory and initialize its database
#[tauri::command]
pub async fn open_project(path: String) -> Result<ProjectInfo, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let project_path = PathBuf::from(&path);

        // Validate path exists
        if !project_path.exists() {
            return Err("Path does not exist".to_string());
        }

        // Validate path is a directory
        if !project_path.is_dir() {
            return Err("Path is not a directory".to_string());
        }

        // Initialize database
        open_project_db(&project_path)
            .map_err(|e| format!("Failed to initialize database: {}", e))?;

        // Get project name from directory name
        let name = project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unnamed Project")
            .to_string();

        Ok(ProjectInfo {
            path: path.clone(),
            name,
            exists: true,
        })
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}
