use super::types::FilePreview;
use leptos::web_sys::{self, File, js_sys};

pub fn bytes_to_blob(file: FilePreview, content: Vec<u8>) -> File {
    let content = content;
    let name = file.filename;
    let mime = file.mime;
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

pub fn format_size(bytes: u64) -> String {
    let size = bytes as f64;

    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;

    match size {
        s if s >= TIB => format!("{:.2} TiB", s / TIB),
        s if s >= GIB => format!("{:.2} GiB", s / GIB),
        s if s >= MIB => format!("{:.2} MiB", s / MIB),
        s if s >= KIB => format!("{:.2} KiB", s / KIB),
        s => format!("{} B", s as u64),
    }
}
