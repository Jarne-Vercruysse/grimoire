use leptos::web_sys::{self, File, js_sys};

use crate::types::FileEntry;

pub fn bytes_to_blob(file: &FileEntry) -> File {
    let content = file.content.clone();
    let name = file.name.clone();
    let mime = file.file_type.clone();
    let uint8_array = js_sys::Uint8Array::from(&content[..]);
    let array = js_sys::Array::new();
    array.push(&uint8_array);
    let options = web_sys::FilePropertyBag::new();
    options.set_type(&mime);

    File::new_with_u8_array_sequence_and_options(&array, &name, &options).unwrap()
}

pub fn download_link(file: File) -> String {
    web_sys::Url::create_object_url_with_blob(&file.into()).unwrap()
}
