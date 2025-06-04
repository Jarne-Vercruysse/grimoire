use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct FileUpload {
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FileRecord {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size: i32,
    pub uploaded_at: chrono::NaiveDateTime,
    pub storage_path: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct NewFileRecord {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size: i32,
    pub storage_path: String,
}
