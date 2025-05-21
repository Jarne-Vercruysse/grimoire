use leptos::wasm_bindgen::{UnwrapThrowExt, closure::Closure};
use leptos::web_sys::wasm_bindgen;
use leptos::web_sys::{self, File, js_sys};
use leptos::{ev, logging};
use std::cell::RefCell;
use std::convert::AsRef;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use crate::types::FileEntry;

pub async fn read_bytes(file: &File) -> Vec<u8> {
    let result = Rc::new(RefCell::new(Vec::new()));
    let result_clone = result.clone();

    let reader = web_sys::FileReader::new().unwrap_throw();
    let read_file = {
        let reader = reader.to_owned();
        Closure::once_into_js(move |_: ev::ProgressEvent| {
            let reader_result = reader.result().unwrap_throw();

            let value = js_sys::Uint8Array::new(&reader_result);
            logging::log!("array lenth: {}", value.clone().length());
            *result_clone.borrow_mut() = vec![0; value.length() as usize];
        })
    };

    reader.set_onloadend(Some(read_file.as_ref().unchecked_ref()));
    reader.read_as_array_buffer(file).unwrap_throw();
    result.borrow().clone()
}

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
