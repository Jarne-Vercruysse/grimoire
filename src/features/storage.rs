use leptos::prelude::*;

#[cfg(feature = "ssr")]
use tokio::fs::{self, File};

#[cfg(feature = "ssr")]
use tokio::io::AsyncWriteExt;

#[cfg(feature = "ssr")]
const UPLOAD_DIR: &str = "./data";

#[server]
pub async fn save_uploaded_file(filename: String, bytes: Vec<u8>) -> Result<String, ServerFnError> {
    // Create full path: e.g. ./data/user/uploads/file.txt
    let folder = format!("{}/user/uploads", UPLOAD_DIR);
    fs::create_dir_all(&folder).await?;

    let full_path = format!("{}/{}", folder, filename);

    let mut file = File::create(&full_path).await?;
    file.write_all(&bytes).await?;

    Ok(full_path)
}
