use std::sync::atomic::{AtomicUsize, Ordering};
use {
    leptos::{html::Div, prelude::*, web_sys::File},
    reactive_stores::{Field, Store},
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Store)]
pub struct UploadTable {
    #[store(key: usize = |file| file.id)]
    pub files: Vec<FileUpload>,
}

impl Default for UploadTable {
    fn default() -> Self {
        Self { files: Vec::new() }
    }
}

#[derive(Store, Clone)]
pub struct FileUpload {
    pub id: usize,
    name: String,
    file_type: String,
    size: f64,
    status: RwSignal<Status>,
}

impl FileUpload {
    pub fn from_web_sys(file: &File) -> Self {
        let name = file.name();
        let file_type = file.type_();
        let size = file.size();
        let status = RwSignal::new(Status::Pending);
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            file_type,
            size,
            status,
        }
    }
}

#[derive(Clone)]
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
        <div node_ref=drop_zone class="border-dashed border-5 border-info grow-1 bg-error">
            <p class="text-5xl">DROP FILES uploadzone</p>
            <div>Dropped: {dropped}</div>
            <div>is_over_drop_zone: {hover}</div>
        </div>
    }
}
