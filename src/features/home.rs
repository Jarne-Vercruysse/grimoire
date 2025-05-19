//use leptos::leptos_dom::logging::console_log;

//use crate::features::upload::UploadTable;

use crate::types::{Client, FileEntry, FileEntryStoreFields, FilesStoreFields, Message};
use reactive_stores::Field;

use super::{
    auth::LogoutUser,
    upload::UploadZone,
    //upload::{FileUpload, FileUploadStoreFields, UploadTableStoreFields, UploadZone},
};
use {
    icondata,
    leptos::{html::Div, logging, prelude::*},
    leptos_icons::Icon,
    leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn},
    //reactive_stores::{Field, Store},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let client = Client::new();
    logging::log!("homepage");

    let (dropped, set_dropped) = signal(false);

    let drop_zone_el = NodeRef::<Div>::new();

    //let files_dropped = move || {
    //    if !files.get().is_empty() {
    //        let _ = files.get().iter().for_each(|drop| {
    //            let file = FileEntry::from(drop);
    //            client.update(Message::Add { entry: file });
    //        });
    //    };
    //};

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
        let _ = files.get();
        if !files.get().is_empty() {
            let files = files
                .get()
                .iter()
                .map(|drop| FileEntry::from(drop))
                .collect::<Vec<FileEntry>>();
            for file in files {
                client.update(Message::Add { entry: file });
            }
        };
    });

    //let upload_data: Store<UploadTable> = Store::new(UploadTable::default());
    //let upload_store = Store::new(UploadTable {
    //    files: (move || {
    //        files
    //            .get()
    //            .iter()
    //            .map(|file| FileUpload::from_web_sys(file))
    //            .collect::<Vec<FileUpload>>()
    //    })(),
    //});
    //
    //Effect::new(move || {
    //    let dropped_files = files.get();
    //    if !dropped_files.is_empty() {
    //        let files = dropped_files
    //            .iter()
    //            .map(|file| {
    //                logging::log!("{}", file.name());
    //                FileUpload::from_web_sys(file)
    //            })
    //            .collect::<Vec<FileUpload>>();
    //        for file in files {
    //            upload_data.files().write().push(file);
    //        }
    //    }
    //});
    //

    view! {
        <div class="h-screen flex flex-row-reverse border-5 border-accent bg-base-100">
            <Sidebar />
            <div class="border-5 w-screen border-secondary flex flex-col">
                <UploadZone drop_zone=drop_zone_el dropped hover=is_over_drop_zone />
                <div class="border-4 grow-3 overflow-auto">
                    <Table client />
                </div>
                <div class="border-4 border-accent">
                    "Action div only visable when something is selected"
                </div>
            </div>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let logout_action = ServerAction::<LogoutUser>::new();

    view! {
        <div class="border-5 border-error p-4 w-64 flex flex-col justify-between bg-base-300">
            <div>
                <Icon icon=icondata::AiApiFilled width="9em" height="9em" />
                <h1 class="text-5xl font-bold">Grimoire</h1>
            </div>

            <ActionForm action=logout_action>
                <input type="submit" value="Logout" class="btn btn-accent mt-4 w-full" />
            </ActionForm>
        </div>
    }
}

#[component]
fn Table(client: Client) -> impl IntoView {
    view! {
        <table class="table table-pin-rows">
            <TableHeader />
            <tbody>
                <For each=move || client.store.0.entries() key=|file| file.id().get() let:file>
                    <FileRow client file />
                </For>
            </tbody>
        </table>
    }
}
#[component]
fn TableHeader() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th>
                    <label>
                        <input type="checkbox" class="checkbox" />
                    </label>
                </th>
                <th>Filename</th>
                <td>Filetype</td>
                <td>Size</td>
                <td>Status</td>
                <td>Actions</td>
            </tr>
        </thead>
    }
}

#[component]
fn FileRow(client: Client, #[prop(into)] file: Field<FileEntry>) -> impl IntoView {
    let remove_handler = move |_| {
        client.update(Message::Remove {
            id: file.id().get(),
        })
    };

    view! {
        <tr class="hover:bg-base-200">
            <td>
                <label>
                    <input type="checkbox" class="checkbox" />
                </label>
            </td>
            <td>{file.name()}</td>
            <td>{file.file_type()}</td>
            <td>{file.get().format_size()}</td>
            <td><div class="badge badge-neutral">Pending</div></td>
            <td>
                <button class="btn" on:click=remove_handler>
                    "X"
                </button>
            </td>
        </tr>
    }
}
