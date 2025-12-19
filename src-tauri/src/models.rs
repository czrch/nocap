#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct MediaFile {
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub media_type: MediaType,
    pub file_size: Option<i64>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub thumbnail_path: Option<String>,
}
