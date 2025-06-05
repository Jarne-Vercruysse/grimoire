use std::collections::HashMap;
use uuid::Uuid;

use gloo::file::{
    FileList,
    callbacks::{self, FileReader},
};
use leptos::{
    html::{A, Input},
    logging::{self, log},
    prelude::*,
    reactive::spawn_local,
    web_sys,
};

use crate::{
    core::{
        types::{AppState, FileAction, FilePreview},
        utils::{bytes_to_blob, download_link},
    },
    features::storage::api::{delete_file, get_file_content, handle_upload},
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
            let id = Uuid::new_v4();
            let form = web_sys::FormData::new().unwrap();
            form.append_with_blob_and_filename(
                "file",
                AsRef::<web_sys::Blob>::as_ref(&file),
                &file.name(),
            )
            .unwrap();
            form.set_with_str("file_id", &id.to_string()).unwrap();
            spawn_local(async move {
                // TODO: handle error
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
                        let file_preview = FilePreview::from_gloo(id, blob, content);
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
        //TODO: Change ICON to loadingicon prob would need a noderef for that
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
pub fn DownloadButton(id: Uuid) -> impl IntoView {
    let a_ref = NodeRef::<A>::new();
    let state = expect_context::<AppState>();

    let on_click = move |_| {
        let node = a_ref.get().expect("a_ref to be mounted");
        if let Some(file) = state.files.find(&id) {
            let file = file.get_untracked();
            spawn_local(async move {
                if let Ok(stored_file) = get_file_content(file.id, file.clone().filename).await {
                    let blob = bytes_to_blob(file, stored_file.content);
                    let url = download_link(blob);
                    node.set_href(&url);
                    node.click();
                }
            });
        }
    };

    view! {
        <button on:click=on_click class="btn btn-ghost btn-xs">
            D
        </button>
        <a node_ref=a_ref download class="hidden"></a>
    }
}

#[component]
pub fn DownloadPageButton(id: Uuid) -> impl IntoView {
    let a_ref = NodeRef::<A>::new();
    let state = expect_context::<AppState>();

    let on_click = move |_| {
        let node = a_ref.get().expect("a_ref to be mounted");
        if let Some(file) = state.files.find(&id) {
            let file = file.get_untracked();
            spawn_local(async move {
                if let Ok(stored_file) = get_file_content(file.id, file.clone().filename).await {
                    let blob = bytes_to_blob(file, stored_file.content);
                    let url = download_link(blob);
                    node.set_href(&url);
                    node.click();
                }
            });
        }
    };

    view! {
        <button on:click=on_click class="btn btn-primary btn-wide">
            Download
        </button>
        <a node_ref=a_ref download class="hidden"></a>
    }
}

#[component]
pub fn ShareButton(#[prop(into)] id: Uuid) -> impl IntoView {
    let url = format!("/download/{}", id);

    view! {
        <a href=url class="btn btn-ghost btn-xs">
            L
        </a>
    }
}
