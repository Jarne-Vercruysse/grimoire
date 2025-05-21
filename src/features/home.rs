use crate::{
    func::{bytes_to_blob, download_link},
    types::{Client, FileEntry, FileEntryStoreFields, FilesStoreFields, Message},
};
use reactive_stores::Field;

use super::{auth::LogoutUser, upload::UploadZone};
use {
    icondata,
    leptos::{html::Div, logging, prelude::*},
    leptos_icons::Icon,
    leptos_use::{UseDropZoneOptions, UseDropZoneReturn, use_drop_zone_with_options},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let client = Client::new();
    logging::log!("homepage");

    let (_dropped, set_dropped) = signal(false);

    let drop_zone_el = NodeRef::<Div>::new();

    let UseDropZoneReturn {
        is_over_drop_zone: _,
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

    view! {
        <div class="h-screen flex flex-row-reverse bg-base-100">
            <Sidebar />
            <div class="flex-1 flex flex-col overflow-hidden">
                <UploadZone drop_zone=drop_zone_el client />
                <div class="overflow-auto px-6 pb-28">
                    <Table client />
                </div>
                // <div class="sticky bottom-0 left-0 w-full bg-base-300 p-4 shadow-md flex justify-end items-center transition-opacity duration-300">
                // <button class="btn btn-primary">Download Selected</button>
                // </div>
                <ActionBar />
            </div>
        </div>
    }
}

#[component]
fn ActionBar() -> impl IntoView {
    view! {
        <div class="fixed bottom-0 right-64 w-full z-50 bg-base-300 p-4 border-t border-base-100 shadow-md">
            // class="w-full bg-base-300 p-4 border-t border-base-100 shadow-md z-50"
            // class:hidden=!(selected_files.get().len() > 0) // Leptos reactive visibility
            <div class="max-w-screen-xl mx-auto flex justify-start">
                <button class="btn btn-primary">Download Selected</button>
            </div>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let logout_action = ServerAction::<LogoutUser>::new();

    view! {
        <div class="w-64 bg-base-300 p-6 flex flex-col justify-between">
            <div class="text-center space-y-4">
                <Icon icon=icondata::AiApiFilled width="9em" height="9em" />
                // <img src="/logo.svg" alt="Grimoire Logo" class="mx-auto h-12" />
                <h1 class="text-2xl font-bold text-base-content">Grimoire</h1>
            </div>
            <label class="toggle text-base-content">
                <input type="checkbox" value="smartburn" class="theme-controller" />
                <svg aria-label="moon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <g
                        stroke-linejoin="round"
                        stroke-linecap="round"
                        stroke-width="2"
                        fill="none"
                        stroke="currentColor"
                    >
                        <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
                    </g>
                </svg>
                <svg aria-label="sun" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <g
                        stroke-linejoin="round"
                        stroke-linecap="round"
                        stroke-width="2"
                        fill="none"
                        stroke="currentColor"
                    >
                        <circle cx="12" cy="12" r="4"></circle>
                        <path d="M12 2v2"></path>
                        <path d="M12 20v2"></path>
                        <path d="m4.93 4.93 1.41 1.41"></path>
                        <path d="m17.66 17.66 1.41 1.41"></path>
                        <path d="M2 12h2"></path>
                        <path d="M20 12h2"></path>
                        <path d="m6.34 17.66-1.41 1.41"></path>
                        <path d="m19.07 4.93-1.41 1.41"></path>
                    </g>
                </svg>

            </label>

            <ActionForm action=logout_action>
                <input type="submit" value="Logout" class="btn btn-error btn-block mt-4" />
            </ActionForm>
        </div>
    }
}

#[component]
fn Table(client: Client) -> impl IntoView {
    view! {
        <table class="table w-full bg-base-100 shadow rounded-box">
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
        <thead class="bg-base-300 text-base-content">
            <tr>
                <th>
                    <label>
                        <input type="checkbox" class="checkbox-sm checkbox" />
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
    let blob = bytes_to_blob(&file.get_untracked());
    let url = download_link(blob);
    //let (link, set_link) = signal(download_link(bytes_to_blob(client.store.0.)));
    let remove_handler = move |_| {
        client.update(Message::Remove {
            id: file.id().get(),
        })
    };

    view! {
        <tr class="hover:bg-base-200">
            <td>
                <label>
                    <input type="checkbox" class="checkbox checkbox-sm" />
                </label>
            </td>
            <td>{file.name()}</td>
            <td>{file.file_type()}</td>
            <td>{file.get().format_size()}</td>
            <td>
                <div class="badge badge-neutral">Pending</div>
            </td>
            <td>
                <button class="btn btn-sm btn-outline" on:click=remove_handler>
                    "Delete"
                </button>

                <a href=url download class="btn btn-sm btn-outline" on:click=remove_handler>
                    "Download"
                </a>
            </td>
        </tr>
    }
}
