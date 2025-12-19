#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
}
