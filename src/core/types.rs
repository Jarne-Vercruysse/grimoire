use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::upload::types::FileRecord;

pub struct User {
    pub id: Uuid,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Clone)]
pub struct AppState {
    pub files: FileState, //files: Store<FilePreviews>,
}

#[derive(Debug, Clone, Copy)]
pub struct FileState(pub Store<FilePreviews>);

#[derive(Debug, Clone, Store, PartialEq, Eq, Default)]
pub struct FilePreviews {
    #[store(key: Uuid= |file| file.id)]
    pub files: Vec<FilePreview>,
}

#[derive(Debug, Clone, Store, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilePreview {
    pub id: Uuid,
    pub filename: String,
    pub mime: String,
    pub size: i32,
    //pub state: FileUploadState,
}

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub enum FileUploadState {
//     NotStarted,
//     Uploading,
//     Completed,
//     Failed(String),
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileAction {
    Add(FilePreview),
    Remove(Uuid),
    Download(Uuid),
}

impl FilePreview {
    pub fn from_record(file: FileRecord) -> Self {
        Self {
            id: file.id,
            filename: file.filename,
            mime: file.mime_type,
            size: file.size,
        }
    }
}

impl FileState {
    pub fn apply_action(&self, action: FileAction) {
        match action {
            FileAction::Add(file) => self.0.files().write().push(file),
            FileAction::Remove(id) => {
                self.0.files().write().retain(|file| file.id != id);
            }
            FileAction::Download(_id) => todo!(),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            files: FileState(Store::new(FilePreviews::default())),
        }
    }

    pub fn update_file_state(&self, action: FileAction) {
        self.files.apply_action(action);
    }
}
