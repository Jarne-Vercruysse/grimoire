use crate::{
    components::{header::Header, row::Row},
    core::types::{
        AppState, FileAction, FilePreview, FilePreviewStoreFields, FilePreviewsStoreFields,
    },
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    use uuid::Uuid;

    let dummy_files = vec![
        FilePreview {
            id: Uuid::new_v4(),
            filename: "resume.pdf".to_string(),
            mime: "application/pdf".to_string(),
            size: 234_567,
        },
        FilePreview {
            id: Uuid::new_v4(),
            filename: "photo.jpg".to_string(),
            mime: "image/jpeg".to_string(),
            size: 3_456_789,
        },
        FilePreview {
            id: Uuid::new_v4(),
            filename: "report.xlsx".to_string(),
            mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
            size: 789_123,
        },
        FilePreview {
            id: Uuid::new_v4(),
            filename: "video.mp4".to_string(),
            mime: "video/mp4".to_string(),
            size: 25_000_000,
        },
        FilePreview {
            id: Uuid::new_v4(),
            filename: "script.js".to_string(),
            mime: "application/javascript".to_string(),
            size: 56_321,
        },
    ];

    let state = expect_context::<AppState>();
    for f in dummy_files {
        state.update_file_state(FileAction::Add(f));
    }

    view! {
        <div class="p-6 space-y-6">
            <div>
                <Header />
                <For each=move || state.files.0.files() key=|file| file.id().get() let:file>
                    <Row file />
                </For>
            </div>

            <div>
                <h2 class="text-lg font-bold text-gray-700 mb-3">Shared with me</h2>
                <div class="text-sm text-gray-500 italic">No shared libraries</div>
            </div>

            <div>
                <h2 class="text-lg font-bold text-gray-700 mb-3">Shared with all</h2>
                <div class="text-sm text-gray-500 italic">No public libraries</div>
            </div>
        </div>
    }
}
