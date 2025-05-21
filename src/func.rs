use leptos::ev;
use leptos::prelude::StorageAccess;
use leptos::wasm_bindgen::{UnwrapThrowExt, closure::Closure};
use leptos::web_sys::{self, File, js_sys};
use std::convert::AsRef;
use wasm_bindgen::JsCast;

use crate::types::FileEntry;

pub fn read_bytes(file: &File) -> Vec<u8> {
    let mut result = Vec::new();

    let reader = web_sys::FileReader::new().unwrap_throw();
    let read_file = {
        let reader = reader.to_owned();
        Closure::once_into_js(move |_: ev::ProgressEvent| {
            let reader_result = reader.result().unwrap_throw();

            let value = js_sys::Uint8Array::new(&reader_result);
            result = vec![0; value.length() as usize];
        })
    };

    reader.set_onloadend(Some(read_file.as_ref().unchecked_ref()));
    //reader.set_onloadend(None);
    //let _ = reader.read_as_array_buffer(file);
    result

    //let read_file = {
    //            // FileReader is cloned prior to moving into the closure
    //            let reader = reader.to_owned();
    //            Closure::once_into_js(move |_: ev::ProgressEvent| {
    //                // `.result` valid after the `read_*` completes on FileReader
    //                // https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result
    //                let result = reader.result().unwrap_throw();
    //                let vec_of_u8_bytes = Uint8Array::new(&result).to_vec();
    //                // Do whatever you want with the Vec<u8>
    //                let content = String::from_utf8(vec_of_u8_bytes).unwrap_throw();
    //                file_signal.update(|items| items.push(Upload { name, content }))
    //            })
    //        };
    //        reader.set_onloadend(Some(read_file.as_ref().unchecked_ref()));
    //
    //        // read_as_array_buffer takes a &Blob
    //        //
    //        // Per https://w3c.github.io/FileAPI/#file-section
    //        // > A File object is a Blob object with a name attribute..
    //        //
    //        // File is a subclass (inherits) from the Blob interface, so a File
    //        // can be used anywhere a Blob is required.
    //        reader.read_as_array_buffer(&file).unwrap_throw();
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
