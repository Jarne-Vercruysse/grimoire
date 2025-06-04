use crate::{
    components::{header::Header, row::Row},
    core::types::{AppState, FilePreviewStoreFields, FilePreviewsStoreFields},
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <div class="mt-10 px-4 sm:px-6 lg:px-8 max-w-5xl mx-auto">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-bold text-base-content">My Files</h2>
                <div class="space-x-2">
                    <button class="btn btn-sm btn-primary">+ Upload</button>
                    <button class="btn btn-sm btn-disabled">Create download link</button>
                </div>
            </div>
            <div class="border border-base-300 rounded-lg overflow-hidden shadow-sm">
                <div class="max-h-[65vh] overflow-y-auto divide-y divide-base-200 animate-fade-in">
                    <Header />
                    <For each=move || state.files.0.files() key=|file| file.id().get() let:file>
                        <Row file />
                    </For>
                </div>
            </div>
        </div>
    }
}
