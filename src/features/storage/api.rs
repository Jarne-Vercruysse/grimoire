use leptos::server;
use server_fn::{
    ServerFnError,
    codec::{MultipartData, MultipartFormData},
};
use uuid::Uuid;

use super::types::FileDownload;
use crate::{core::types::FilePreview, features::storage::types::FileStorage};

#[cfg(feature = "ssr")]
use super::server::*;
#[cfg(feature = "ssr")]
use super::types::NewFileRecord;
#[cfg(feature = "ssr")]
use leptos::logging;
#[cfg(feature = "ssr")]
use std::io::Error;

#[server]
pub async fn load_users_files() -> Result<Vec<FilePreview>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let mut files: Vec<FilePreview> = Vec::new();
        if let Ok(records) = get_files().await {
            files = records
                .iter()
                .map(|file| FilePreview::from_record(file.clone()))
                .collect();
        }
        return Ok(files);
    }
}

#[server]
pub async fn delete_file(id: Uuid) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if let Err(err) = get_file(id).await {
            return Err(ServerFnError::ServerError(err.to_string()));
        }
        if let Err(err) = delete_file_in_storage(id).await {
            return Err(ServerFnError::ServerError(err.to_string()));
        }
        if let Err(err) = delete_file_in_db(id).await {
            return Err(ServerFnError::ServerError(err.to_string()));
        }
        return Ok(());
    }
}

#[server(input=MultipartFormData,)]
pub async fn handle_upload(files: MultipartData) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let mut id: Option<Uuid> = None;
        let mut filename: String = String::new();
        let mut mime: String = String::new();
        let mut content: Option<Vec<u8>> = None;

        let mut files = files.into_inner().unwrap();
        while let Ok(Some(field)) = files.next_field().await {
            match field.name() {
                Some("file_id") => {
                    let file_id = field.text().await?;
                    id = Some(Uuid::parse_str(&file_id)?);
                }
                Some("file") => {
                    filename = field.file_name().unwrap_or_default().to_string();
                    mime = field.content_type().unwrap().to_string();
                    content = Some(field.bytes().await?.to_vec());
                }
                _ => {}
            }
        }

        let id = id.ok_or_else(|| ServerFnError::<Error>::ServerError("Missing file id".into()))?;
        let content = content
            .ok_or_else(|| ServerFnError::<Error>::ServerError("Missing file content".into()))?;

        let size = content.len();
        let file = FileStorage {
            filename: filename.clone(),
            content,
            id,
        };

        if let Ok(storage_path) = save_uploaded_file(file.clone()).await {
            logging::log!("Stored in: {}", storage_path.clone());
            let file = NewFileRecord {
                id,
                filename,
                mime_type: mime,
                size: size as i64,
                storage_path,
            };

            if let Err(_) = store_in_db(file).await {
                return Err(ServerFnError::ServerError(
                    "Failed to store in db".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[server]
pub async fn get_file_content(id: Uuid, filename: String) -> Result<FileStorage, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if let Ok(file) = get_stored_file(id, filename).await {
            return Ok(file);
        }
        return Err(ServerFnError::ServerError("not found".to_owned()));
    }
}

#[server]
pub async fn fetch_download_file_by_id(id: Uuid) -> Result<FileDownload, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if let Ok(record) = get_file(id).await {
            if let Ok(file) = get_stored_file(id, record.filename.clone()).await {
                let download = FileDownload {
                    id,
                    filename: record.filename,
                    mime_type: record.mime_type,
                    size: record.size as u64,
                    content: file.content,
                };

                return Ok(download);
            }
        }

        return Err(ServerFnError::ServerError("not found".to_owned()));
    }
}
