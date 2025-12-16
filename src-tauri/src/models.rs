#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageMetadata {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub size: u64,
    pub format: String,
}
