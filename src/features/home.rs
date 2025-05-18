use leptos::leptos_dom::logging::console_log;

use crate::features::upload::UploadTable;

use super::{
    auth::LogoutUser,
    upload::{FileUpload, FileUploadStoreFields, UploadTableStoreFields, UploadZone},
};
use {
    icondata,
    leptos::{html::Div, logging, prelude::*},
    leptos_icons::Icon,
    leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn},
    reactive_stores::{Field, Store},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let logout_action = ServerAction::<LogoutUser>::new();
    logging::log!("homepage");

    let (dropped, set_dropped) = signal(false);

    let drop_zone_el = NodeRef::<Div>::new();

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default()
            .on_drop(move |_| set_dropped(true))
            .on_enter(move |_| set_dropped(false)),
    );

    let upload_data: Store<UploadTable> = Store::new(UploadTable::default());
    let upload_store = Store::new(UploadTable {
        files: (move || {
            files
                .get()
                .iter()
                .map(|file| FileUpload::from_web_sys(file))
                .collect::<Vec<FileUpload>>()
        })(),
    });

    Effect::new(move || {
        let dropped_files = files.get();
        if !dropped_files.is_empty() {
            let files = dropped_files
                .iter()
                .map(|file| FileUpload::from_web_sys(file))
                .collect::<Vec<FileUpload>>();
            for file in files {
                upload_store.files().write().push(file);
            }
        }
    });

    view! {
        <div class="min-h-screen flex flex-row-reverse border-5 border-accent">
            <div class="border-5 border-error p-10 flex flex-col justify-between">
                <div>
                    <Icon icon=icondata::AiApiFilled width="9em" height="9em" />
                    // <img src=icondata::AiApiFilled/>
                    <h1 class="text-5xl font-bold">Grimoire</h1>
                </div>

                <ActionForm action=logout_action>
                    <input type="submit" value="Logout" class="btn btn-accent mt-4 w-full" />
                </ActionForm>
            </div>

            // div for Main content
            // "Main content"
            <div class="border-5 border-secondary flex flex-col w-screen">
                // div around drop zone

                <UploadZone drop_zone=drop_zone_el dropped hover=is_over_drop_zone />
                // <div class="border-dashed border-5 bg-base-100 p-20 border border-secondary glass grow-1">
                // <p class="text-5xl">DROP FILES</p>
                // </div>
                // div aroun table
                <div class="border-4 border-primary grow-5">
                    <table class="table bg-base-300">
                        <thead>
                            <tr>
                                <th>
                                    <label>
                                        <input type="checkbox" class="checkbox" />
                                    </label>
                                </th>
                                <th>Name</th>
                                <td>Type</td>
                                <td>Size</td>
                            </tr>
                        </thead>
                        <tbody>
                            <For each=move || upload_store.files() key=|f| f.id().get() let:file>
                                <FileRow file />

                            </For>

                            <tr class="hover:bg-base-300">
                                <td>
                                    <label>
                                        <input type="checkbox" class="checkbox" />
                                    </label>
                                </td>
                                <td>1Cy Ganderton</td>
                                <td>Quality Control Specialist</td>
                                <td>Blue</td>
                            </tr>

                        </tbody>
                    </table>
                </div>
                <div class="border-4 border-accent">
                    "Action div only visable when something is selected"
                </div>
            </div>
        </div>
    }
}
#[component]
fn FileRow(#[prop(into)] file: Field<FileUpload>) -> impl IntoView {
    view! {
        <tr class="hover:bg-base-300">
            <td>
                <label>
                    <input type="checkbox" class="checkbox" />
                </label>
            </td>
            <td>{file.name()}</td>
            <td>{file.file_type()}</td>
            <td>{file.size()}</td>
        </tr>
    }
}
