use std::collections::HashMap;

#[cfg(feature = "ssr")]
use super::storage::save_uploaded_file;
use super::storage::store_in_db;
use crate::types::Message;
use crate::types::{Client, FileEntry};
use chrono;
use gloo::file::{
    FileList,
    callbacks::{self, FileReader},
};
use icondata;
use leptos::task::spawn_local;
use leptos::web_sys::FormData;
use leptos::{
    html::{Div, Input},
    logging,
    prelude::*,
};
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};
use server_fn::codec::{MultipartData, MultipartFormData};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct FileUpload {
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FileRecord {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size: i32,
    pub uploaded_at: chrono::NaiveDateTime,
    pub storage_path: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewFileRecord {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size: i32,
    pub storage_path: String,
}

impl NewFileRecord {
    pub fn new(
        id: Uuid,
        filename: String,
        mime_type: String,
        size: i32,
        storage_path: String,
    ) -> Self {
        Self {
            id,
            filename,
            mime_type,
            size,
            storage_path,
        }
    }
}

#[server(input=MultipartFormData,)]
pub async fn upload_file(data: MultipartData) -> Result<(), ServerFnError> {
    let mut data = data.into_inner().unwrap();
    let id = Uuid::new_v4();

    while let Ok(Some(field)) = data.next_field().await {
        let file_name = field.file_name().unwrap_or_default().to_string();
        let mime_type = field.content_type().unwrap().to_string();
        let bytes = field.bytes().await?;
        let size = bytes.len() as i32;

        // TODO: Handle errors
        let path = save_uploaded_file(file_name.clone(), bytes.to_vec(), id)
            .await
            .unwrap();

        logging::log!("File in path: {}", path.clone());

        let file = NewFileRecord::new(id, file_name, mime_type, size, path);
        let result = store_in_db(file).await;

        match result {
            Ok(_) => {
                logging::log!("Stored in DB and in local storage")
            }
            Err(_) => {
                logging::log!("Got this error:")
            }
        }
    }

    Ok(())
}

#[component]
pub fn UploadZone(drop_zone: NodeRef<Div>, client: Client) -> impl IntoView {
    let file_input = NodeRef::<Input>::new();
    let (_, set_readers) = signal_local(HashMap::<String, FileReader>::new());

    let file_handler = move |_| {
        let files = FileList::from(file_input.get().unwrap().files().unwrap());

        logging::log!("Will read {} files", files.len());
        files.iter().for_each(move |blob| {
            let file = blob.clone();
            let reader = callbacks::read_as_bytes(blob, move |result| {
                logging::log!("Callback fired");
                match result {
                    Ok(bytes) => {
                        logging::log!("bytes: {}", bytes.len());
                        let entry = FileEntry::from(file.clone(), bytes);
                        client.update(Message::Add { entry });

                        // FIX: Should move this out of the callback
                        let form = FormData::new().unwrap();
                        form.append_with_blob_and_filename(
                            "file",
                            AsRef::<leptos::web_sys::Blob>::as_ref(&file),
                            &file.name(),
                        )
                        .unwrap();
                        spawn_local(async move {
                            let _ = upload_file(form.into()).await;
                        });
                    }
                    Err(err) => {
                        logging::log!("Err: {}", err)
                    }
                }
            });

            set_readers.update(|map| {
                map.insert(blob.name(), reader);
            });
        });
    };

    view! {
        <div class="p-6">
            <div
                node_ref=drop_zone
                class=" relative
                border-2 border-dashed border-base-content/40 bg-base-100 rounded-xl h-64 flex flex-col justify-center items-center text-center space-y-2 hover:bg-base-200 transition
                "
            >
                <svg class="w-12 h-12 text-primary" fill="none" stroke="currentColor">
                    <Icon icon=icondata::FiFilePlus width="2em" height="2em" />
                </svg>
                <h2 class="text-lg font-semibold text-base-content">Upload your files</h2>
                <p class="text-sm text-base-content/70">
                    <span class="underline">Click to browse</span>
                    or drag & drop them here
                </p>

                <input
                    type="file"
                    node_ref=file_input
                    multiple
                    on:change=file_handler
                    class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                />

            </div>
        </div>
    }
}
