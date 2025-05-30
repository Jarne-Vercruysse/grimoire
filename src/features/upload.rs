use crate::types::Client;
use gloo::file::{FileList, callbacks};
use icondata;
use leptos::{
    html::{Div, Input},
    logging,
    prelude::*,
};
use leptos_icons::Icon;

// pub async fn upload_files(mut multipart: Multipart) -> impl IntoResponse {
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap();
//         println!("Uploading a file: {}", name);
//     }
//     StatusCode::OK
// }

#[component]
pub fn UploadZone(drop_zone: NodeRef<Div>, client: Client) -> impl IntoView {
    let file_input = NodeRef::<Input>::new();

    let file_handler = move |_| {
        let files = FileList::from(file_input.get().unwrap().files().unwrap());
        files.iter().for_each(move |blob| {
            callbacks::read_as_bytes(blob, move |result| match result {
                Ok(bytes) => {
                    logging::log!("bytes: {}", bytes.len())
                }
                Err(err) => {
                    logging::log!("Err: {}", err)
                }
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
