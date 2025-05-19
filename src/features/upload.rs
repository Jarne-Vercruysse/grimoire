use icondata;
use leptos::tachys::dom::{event_target_checked, event_target_value};
use leptos::web_sys;
use leptos::web_sys::HtmlElement;
use leptos::{
    html::{Div, Input},
    logging,
    prelude::*,
};
use leptos_icons::Icon;

use crate::types::{Client, FileEntry, Message};

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
pub fn UploadZone(drop_zone: NodeRef<Div>, client: Client) -> impl IntoView {
    //let add_files = NodeRef::<Input>::new();
    //let file_handler = move |_| {
    //add_files.get();
    //let files = add_files.files.expect("Should be mounted").value();
    //};
    let input_ref = NodeRef::<Input>::new();
    let file_handler = move |_| {
        let files = input_ref.get().unwrap().files().unwrap();
        let length = files.length();
        for f in 0..length {
            logging::log!("Index: {}", f);
            logging::log!("filelist: {:#?}", files.item(f));
            let file = files.item(f).unwrap();
            client.update(Message::Add {
                entry: FileEntry::from(&file),
            });
            //logging::log!("file: {:#?}", file.name());
        }
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
               // <div class="space-y-2">
               <Icon icon=icondata::FiFilePlus width="5em" height="5em" />
               // <div class="text-6xl">ICON</div>
               <h2 class="text-2xl font-semibold">Upload your files</h2>
               <p class="text-sm opacity-60">
                   <span class="underline">Click to browse</span>
                   or drag & drop them here
               </p>

               // <input type="file" multiple on:drop=drop_handler class="btn btn-neutral"/>

               <input type="file" node_ref=input_ref multiple on:change=file_handler
           //class="btn btn-neutral"
          class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
    />
           // on:change=move |ev| {
           // logging::log!("penis");
           // }
           // class="btn btn-neutral"
           // class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
           // <input
           // type="file"
           // //node_ref=add_files
           // multiple
           // // on:change=file_handler
           // class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
           // />
           // // </div>
           </div>
       }
}

//#[component]
//pub fn UploadZone(drop_zone: NodeRef<Div>) -> impl IntoView {
//    let drop_handler = move |_| logging::log!("drop_handler");
//    view! {
//        <div
//            node_ref=drop_zone
//            class=" relative w-full min-h-64 border-2 border-dashed transition-all duration-300
//            flex  flex-col items-center justify-center text-center select-none cursor-pointer rounded-2xl
//            group px-6 bg-base-100 space-y-2
//            hover:shadow-lg hover:bg-base-200 hover:border-primary text-base-content
//            "
//        >
//            // <div class="space-y-2">
//            <Icon icon=icondata::FiFilePlus width="5em" height="5em" />
//            // <div class="text-6xl">ICON</div>
//            <h2 class="text-2xl font-semibold">Upload your files</h2>
//            <p class="text-sm opacity-60">
//                <span class="underline">Click to browse</span>
//                or drag & drop them here
//            </p>
//
//            // <input type="file" multiple on:drop=drop_handler class="btn btn-neutral"/>
//
//            <input
//                type="file"
//                multiple
//                on:change=drop_handler
//                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
//            />
//        // </div>
//        </div>
//    }
//}
