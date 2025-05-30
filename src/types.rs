use std::u8;

use leptos::prelude::*;
use reactive_stores::{ArcStore, Field, Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Store, PartialEq, Eq)]
pub struct Files {
    #[store(key: Uuid = |file| file.id)]
    pub entries: Vec<FileEntry>,
}

#[derive(Debug, Clone, Store, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Store, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Connect,
    Disconnect,
    Welcome { list: Vec<FileEntry> },
    Add { entry: FileEntry },
    Remove { id: Uuid },
    MarkComplete { id: Uuid, completed: bool },
    Edit { id: Uuid, entry: FileEntry },
    Download { id: Uuid },
}

#[derive(Debug, Default, Clone, Copy)]
pub struct User {
    pub id: Uuid,
    //pub first_name: String,
    //pub last_name: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Client {
    pub store: State,
    pub user: User,
}

#[derive(Debug, Clone, Copy)]
pub struct State(pub Store<Files>);

impl From<ArcStore<Files>> for State {
    fn from(value: ArcStore<Files>) -> Self {
        State(value.into())
    }
}

impl State {
    pub fn apply_local_update(&self, message: Message) {
        match message {
            Message::Connect => {}
            Message::Disconnect => {}
            Message::Welcome { list } => *self.0.entries().write() = list,
            Message::Add { entry } => self.0.entries().write().push(entry),
            Message::Remove { id } => {
                self.0.entries().write().retain(|file| file.id != id);
            }
            _ => {}
        }
    }

    fn find(&self, id: &Uuid) -> Option<Field<FileEntry>> {
        let store = self.0.entries().read_untracked();
        store
            .iter()
            .position(|file| &file.id == id)
            .map(|idx| self.0.entries().at_unkeyed(idx).into())
    }
}

impl Client {
    pub fn new() -> Self {
        let user = User::default();
        Self {
            user,
            store: State(Store::new(Files::default())),
        }
    }
    pub fn update(&self, message: Message) {
        self.store.apply_local_update(message);
    }
}

impl FileEntry {
    pub fn from(file: &gloo::file::File) -> Self {
        let id = Uuid::new_v4();
        let name = file.name();
        let file_type = file.raw_mime_type();
        let size = file.size() as u64;
        let content = Vec::new();

        Self {
            id,
            name,
            file_type,
            size,
            content,
        }
    }
    pub fn update_content(&mut self, content: Vec<u8>) {
        self.content = content;
    }

    pub fn format_size(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        match self.size {
            b if b >= GB => format!("{:.2} GB", b as f64 / GB as f64),
            b if b >= MB => format!("{:.2} MB", b as f64 / MB as f64),
            b if b >= KB => format!("{:.2} KB", b as f64 / KB as f64),
            b => format!("{} B", b),
        }
    }
}
