use serde::{Deserialize, Serialize};
use uuid::Uuid;

use {
    leptos::{html::Div, logging, prelude::*, web_sys::File},
    reactive_stores::Store,
};

#[derive(Store)]
pub struct UploadTable {
    #[store(key: Uuid = |file| file.id)]
    pub files: Vec<FileUpload>,
}

impl Default for UploadTable {
    fn default() -> Self {
        Self { files: Vec::new() }
    }
}

#[derive(Store, Clone)]
pub struct FileUpload {
    pub id: Uuid,
    pub name: String,
    file_type: String,
    size: f64,
    pub status: Status,
    //status: RwSignal<Status>,
}

impl FileUpload {
    pub fn from_web_sys(file: &File) -> Self {
        let name = file.name();
        let file_type = file.type_();
        let size = file.size();
        let status = Status::Pending;
        let id = Uuid::new_v4();
        Self {
            id,
            name,
            file_type,
            size,
            status,
        }
    }

    pub fn update_status(&mut self, status: Status) {
        match status {
            Status::Pending => self.status = Status::Pending,
            Status::Uploading => self.status = Status::Uploading,
            Status::Uploaded => self.status = Status::Uploaded,
            Status::Failed => self.status = Status::Failed,
            Status::Cancelled => self.status = Status::Cancelled,
        };
    }
    pub fn to_dto(&self) -> FileUploadDto {
        FileUploadDto {
            id: self.id,
            name: self.name.clone(),
            file_type: self.file_type.clone(),
            size: self.size,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileUploadDto {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
    pub size: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Pending,
    Uploading,
    Uploaded,
    Failed,
    Cancelled,
}

impl Status {
    pub fn badge_props(status: Status) -> (&'static str, &'static str) {
        match status {
            Status::Pending => ("Pending", "badge badge-info"),
            Status::Uploading => ("Uploading", "badge badge-info"),
            Status::Uploaded => ("Uploaded", "badge badge-succes"),
            Status::Failed => ("Failed", "badge badge-error"),
            Status::Cancelled => ("Cancelled", "badge badge-warning"),
        }
    }
}

#[component]
pub fn UploadZone(
    drop_zone: NodeRef<Div>,
    dropped: ReadSignal<bool>,
    hover: Signal<bool>,
) -> impl IntoView {
    view! {
        <div node_ref=drop_zone class="border-dashed border-5 border-info grow-1 bg-info">
            <p class="text-5xl">DROP FILES uploadzone</p>
            <div>Dropped: {dropped}</div>
            <div>is_over_drop_zone: {hover}</div>
        </div>
    }
}

#[server]
pub async fn upload_file(file: FileUploadDto) -> Result<(), ServerFnError> {
    logging::log!("file uploading: {:?}", file.name);
    let _ = tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    logging::log!("file uploaded: {:?}", file.name);

    Ok(())
}
