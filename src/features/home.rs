use uuid::Uuid;

use crate::features::upload::{upload_file, FileUploadDto};

use super::{
    auth::LogoutUser,
    upload::{
        FileUpload, FileUploadStoreFields, Status, UploadTable, UploadTableStoreFields, UploadZone,
    },
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

    let upload_store = Store::new(UploadTable::default());
    provide_context(upload_store.clone());

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

    Effect::new(move || {
        let dropped_files = files.get();
        if !dropped_files.is_empty() {
            upload_store.update(|table| {
                for file in dropped_files {
                    let file_upload = FileUpload::from_web_sys(&file);
                    table.files.push(file_upload);
                }
            });
        }
    });

    //let upload_data = Store::new(UploadTable::default());
    //Effect::new(move |_| {
    //    let dropped_files = files.get();
    //    if !dropped_files.is_empty() {
    //        let files = dropped_files
    //            .iter()
    //            .map(|file| FileUpload::from_web_sys(file))
    //            .collect::<Vec<FileUpload>>();
    //
    //        for file in files {
    //            upload_data.files().write().push(file);
    //        }
    //    }
    //});

    let trigger_upload = Action::new(move |input: (Uuid, FileUploadDto)| async move {
        let input = input.clone();
        let store = upload_store.clone();

        store.files().update(|files| {
            if let Some(file) = files.iter_mut().find(|f| f.id == input.0) {
                file.update_status(Status::Uploading);
            }
        });
        match upload_file(input.1).await {
            Ok(_) => {
                store.files().update(|files| {
                    if let Some(file) = files.iter_mut().find(|f| f.id == input.0) {
                        file.update_status(Status::Uploaded);
                    }
                });
            }
            Err(_) => {
                store.files().update(|files| {
                    if let Some(file) = files.iter_mut().find(|f| f.id == input.0) {
                        file.update_status(Status::Failed);
                    }
                });
            }
        }
    });

    let trigger_uploads = move || {
        let current_files = upload_store.read().files.clone();
        for file in current_files {
            if file.status == Status::Pending {
                let dto = file.to_dto();
                trigger_upload.dispatch((file.id, dto));
            }
        }
    };

    view! {
        <div class="min-h-screen flex flex-row-reverse border-5 border-accent bg-base-300">
            <div class="border-5 border-error p-10 flex flex-col justify-between bg-base-200">
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
                <h2>UPLOAD data</h2>
                <button class="btn btn-accent" on:click= move |_| trigger_uploads()>
                    "Upload Pending Files"
                </button>
                <div class="border-4 border-primary grow-5">
                    <table class="table bg-base-300">
                        <FileHeader />
                        <tbody>
                            <For
                                each=move || upload_store.files()
                                key=|f| f.clone().id().get()
                                let:file
                            >
                                <FileRow file />

                            </For>

                        // <tr class="hover:bg-base-300">
                        // <td>
                        // <label>
                        // <input type="checkbox" class="checkbox" />
                        // </label>
                        // </td>
                        // <td>1Cy Ganderton</td>
                        // <td>Quality Control Specialist</td>
                        // <td>Blue</td>
                        // </tr>
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
fn FileHeader() -> impl IntoView {
    view! {
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
                <td>Status</td>
            </tr>
        </thead>
    }
}

#[component]
fn FileRow(#[prop(into)] file: Field<FileUpload>) -> impl IntoView {
    let status = file.status();
    logging::log!("Creating a row");
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
            <StatusBadge status />
        </tr>
    }
}

#[component]
fn StatusBadge(#[prop(into)] status: Signal<Status>) -> impl IntoView {
    //let x = status;
    logging::log!("Setting status");
    let badge = Status::badge_props((move || status.get())());
    view! {
        <td>
            <span class=badge.1>{badge.0}</span>
        </td>
    }
}
