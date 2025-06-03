use crate::core::types::{FilePreview, FilePreviewStoreFields};
use leptos::prelude::*;

#[component]
pub fn row(#[prop(into)] file: reactive_stores::Field<FilePreview>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-4 py-3 text-sm items-center border-b hover:bg-base-200 transition">
            <div class="text-primary font-medium">{file.filename()}</div>
            <div>{file.size()}</div>
            <div>{file.mime()}</div>
        </div>
    }
}
