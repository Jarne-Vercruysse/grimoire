use std::collections::HashMap;
use uuid::Uuid;

use gloo::file::{
    FileList,
    callbacks::{self, FileReader},
};
use leptos::{
    html::Input,
    logging::{self, log},
    prelude::*,
    reactive::spawn_local,
    web_sys,
};

use crate::{
    core::types::{AppState, FileAction, FilePreview},
    features::storage::api::{delete_file, handle_upload},
};

#[component]
pub fn UploadButton() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let (_, set_readers) = signal_local(HashMap::<String, FileReader>::new());

    let state = expect_context::<AppState>();

    let on_change = move |_| {
        let files = FileList::from(
            input_ref
                .get()
                .expect("ref should be loaded by now")
                .files()
                .unwrap(),
        );

        files.iter().for_each(move |file| {
            let form = web_sys::FormData::new().unwrap();
            form.append_with_blob_and_filename(
                "file",
                AsRef::<web_sys::Blob>::as_ref(&file),
                &file.name(),
            )
            .unwrap();
            spawn_local(async move {
                let _ = handle_upload(form.into()).await;
            });

            // INFO: Clone the file before the callback, cause files.iter takes a ref to file
            let blob = file.clone();
            let reader = callbacks::read_as_bytes(file, move |result| {
                logging::log!("Callback");
                match result {
                    Ok(content) => {
                        logging::log!("bytes: {}", content.len());
                        // INFO: We need to use the cloned value here otherwise we get a borrowing issue
                        let file_preview = FilePreview::from_gloo(blob, content);
                        state.update_file_state(FileAction::Add { file: file_preview });
                    }
                    Err(err) => logging::log!("Error: {}", err),
                }
            });

            set_readers.update(|map| {
                map.insert(file.name(), reader);
            });
        });
    };

    let on_click = move |_| {
        let node = input_ref.get().expect("input_ref should be loaded by now");
        log!("value is {:?}", node.value());
        node.click();
    };

    view! {
        <button on:click=on_click class="btn btn-sm btn-primary">
            + Upload
        </button>
        <input type="file" node_ref=input_ref multiple class="hidden" on:change=on_change />
    }
}

#[component]
pub fn DeleteButton(id: Uuid) -> impl IntoView {
    let state = expect_context::<AppState>();
    let on_click = move |_| {
        //TODO: Change ICON to loadingicon
        spawn_local(async move {
            let result = delete_file(id).await;
            match result {
                Ok(_) => {
                    state.update_file_state(FileAction::Remove { id });
                }
                Err(err) => {
                    log!("Error: {}", err)
                }
            }
        });
    };

    view! {
        <button on:click=on_click class="btn btn-ghost btn-xs">
            X
        </button>
    }
}

#[component]
pub fn DownloadButton() -> impl IntoView {}
