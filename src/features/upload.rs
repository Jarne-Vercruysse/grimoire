use icondata;
use leptos::{html::Div, prelude::*};
use leptos_icons::Icon;

//#[derive(Store)]
//pub struct UploadTable {
//    #[storerkey: usize = |file| file.id)]
//    pub files: Vec<FileUpload>,
//}
//
//impl Default for UploadTable {
//    fn default() -> Self {
//        Self { files: Vec::new() }
//    }
//}
//
//#[derive(Store, Clone)]
//pub struct FileUpload {
//    pub id: usize,
//    name: String,
//    file_type: String,
//    size: f64,
//}
//
//impl FileUpload {
//    pub fn from_web_sys(file: &File) -> Self {
//        let name = file.name();
//        let file_type = file.type_();
//        let size = file.size();
//        Self {
//            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
//            name,
//            file_type,
//            size,
//        }
//    }
//}

#[component]
pub fn UploadZone(drop_zone: NodeRef<Div>) -> impl IntoView {
    view! {
        <div
            node_ref=drop_zone
            class=" relative w-full min-h-64 border-2 border-dashed transition-all duration-300
            flex  flex-col items-center justify-center text-center select-none cursor-pointer rounded-2xl
            group px-6 bg-base-100 space-y-2
            hover:shadow-lg hover:bg-base-200 hover:border-primary text-base-content
            "
        >
            //<div class="space-y-2">
                <Icon icon=icondata::FiFilePlus width="5em" height="5em" />
                //<div class="text-6xl">ICON</div>
                <h2 class="text-2xl font-semibold">Upload your files</h2>
                <p class="text-sm opacity-60">
                    <span class="underline">Click to browse</span>
                    or drag & drop them here
                </p>
            //</div>
        </div>
    }
}
