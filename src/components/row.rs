use crate::{
    components::button::{DeleteButton, DownloadButton},
    core::{
        types::{FilePreview, FilePreviewStoreFields},
        utils::format_size,
    },
};
use leptos::prelude::*;

#[component]
pub fn row(#[prop(into)] file: reactive_stores::Field<FilePreview>) -> impl IntoView {
    let id = file.id().get();
    let formated_size = format_size(file.size().get_untracked());
    view! {
        <div class="flex items-center gap-x-4 px-4 py-3 text-sm text-base-content hover:bg-base-200 transition-colors duration-150 border-b border-base-300">
            <div class="w-[5%] flex justify-center">
                <input type="checkbox" class="checkbox checkbox-xs" />
            </div>
            <div class="w-[35%] truncate">{file.filename()}</div>
            <div class="w-[15%] text-right">{formated_size}</div>
            <div class="w-[30%] truncate">{file.mime()}</div>
            <div class="w-[15%] text-right space-x-1">
                <DownloadButton id=id />
                <DeleteButton id=id />
            </div>
        </div>
    }
}
