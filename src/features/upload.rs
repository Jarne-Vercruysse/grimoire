use {
    leptos::{html::Div, prelude::*},
    leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn},
    reactive_stores::Store,
};

#[derive(Store)]
struct UploadTable {
    #[store]
    files: Vec<FileUpload>,
}

#[derive(Store, Clone)]
struct FileUpload {
    name: String,
    file_type: String,
    size: f64,
}

impl FileUpload {
    fn new(name: String, file_type: String, size: f64) -> Self {
        Self {
            name,
            file_type,
            size,
        }
    }
}

#[component]
pub fn UploadZone() -> impl IntoView {
    let (dropped, set_dropped) = signal(false);
    let upload_store = Store::new(FileUpload::new(
        "test".to_string(),
        "big".to_string(),
        355.0,
    ));

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

    //let empty_files = files.read().clear();
    //let upload =
    //    upload_store
    //        .files()
    //        .write()
    //        .push(FileUpload::new(file.name(), file.type_(), file.size()));

    view! {
        <div node_ref=drop_zone_el class="border-dashed border-5 border-info grow-1 bg-error">
            <p class="text-5xl">DROP FILES uploadzone</p>
            <div>is_over_drop_zone: {is_over_drop_zone}</div>
            <div>dropped: {dropped}</div>
            <div class="flex flex-wrap justify-center items-center">
                <For each=files key=|f| f.name() let:file>
                    <div class="w-200px bg-black-200/10 ma-2 pa-6">
                        <p>Name: {file.name()}</p>
                        <p>Size: {file.size()}</p>
                        <p>Type: {file.type_()}</p>
                        // <p>Type: {file.type_}</p>
                        <p>Last modified: {file.last_modified()}</p>
                    </div>
                </For>

            </div>
        </div>
    }
}
