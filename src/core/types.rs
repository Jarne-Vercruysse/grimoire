use leptos::prelude::*;
use reactive_stores::{Field, Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::storage::types::FileRecord;

pub struct User {
    pub id: Uuid,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Clone, Copy)]
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
    pub size: u64,
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
    Add { file: FilePreview },
    Remove { id: Uuid },
    Download { id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateAction {
    InitialiseFileState(Vec<FilePreview>),
}

impl FilePreview {
    pub fn from_record(file: FileRecord) -> Self {
        Self {
            id: file.id,
            filename: file.filename,
            mime: file.mime_type,
            size: file.size as u64,
        }
    }
    pub fn from_gloo(id: Uuid, file: gloo::file::File, content: Vec<u8>) -> Self {
        Self {
            id,
            filename: file.name(),
            mime: file.raw_mime_type(),
            size: content.len() as u64,
        }
    }
}

impl FileState {
    pub fn apply_action(&self, action: FileAction) {
        match action {
            FileAction::Add { file } => self.0.files().write().push(file),
            FileAction::Remove { id } => {
                self.0.files().write().retain(|file| file.id != id);
            }
            FileAction::Download { id: _ } => todo!(),
        }
    }

    pub fn find(&self, id: &Uuid) -> Option<Field<FilePreview>> {
        let store = self.0.files().read_untracked();
        store
            .iter()
            .position(|file| &file.id == id)
            .map(|idx| self.0.files().at_unkeyed(idx).into())
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            files: FileState(Store::new(FilePreviews::default())),
        }
    }

    pub fn initialise_state(&self, action: StateAction) {
        match action {
            StateAction::InitialiseFileState(files) => files.iter().for_each(|file| {
                self.files
                    .apply_action(FileAction::Add { file: file.clone() })
            }),
        }
    }

    pub fn update_file_state(&self, action: FileAction) {
        self.files.apply_action(action);
    }
}
