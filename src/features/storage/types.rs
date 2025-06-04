use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStorage {
    pub id: Uuid,
    pub filename: String,
    pub content: Vec<u8>,
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

impl NewFileRecord {
    pub fn new(
        id: Uuid,
        filename: String,
        mime_type: String,
        size: i32,
        storage_path: String,
    ) -> Self {
        Self {
            id,
            filename,
            mime_type,
            size,
            storage_path,
        }
    }
}
