use icondata;
use leptos::{
    html::{Div, Input},
    logging,
    prelude::*,
};
use leptos_icons::Icon;

use crate::types::{Client, FileEntry, Message};

#[component]
pub fn UploadZone(drop_zone: NodeRef<Div>, client: Client) -> impl IntoView {
    let file_input = NodeRef::<Input>::new();
    let file_handler = move |_| {
        let _files = match file_input.get() {
            Some(element) => match element.files() {
                Some(filelist) => {
                    // TODO: Implemeting to iter
                    let lists_length = filelist.length();
                    for index in 0..lists_length {
                        match filelist.item(index) {
                            Some(file) => {
                                let file = FileEntry::from(&file);
                                client.update(Message::Add { entry: file });
                            }
                            None => {
                                logging::warn!("Out of bound indexing");
                            }
                        }
                    }
                }
                None => {
                    logging::warn!("No files found on input element")
                }
            },
            None => {
                logging::warn!("NodeRef not mounted")
            }
        };
    };

    view! {
        <div
            node_ref=drop_zone
            class=" relative w-full min-h-64 border-2 border-dashed transition-all duration-300
            flex  flex-col items-center justify-center text-center select-none cursor-pointer rounded-2xl
            group px-6 bg-base-100 space-y-2
            hover:shadow-lg hover:bg-base-200 hover:border-primary text-base-content
            "
        >
            <Icon icon=icondata::FiFilePlus width="5em" height="5em" />
            <h2 class="text-2xl font-semibold">Upload your files</h2>
            <p class="text-sm opacity-60">
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
    }
}
