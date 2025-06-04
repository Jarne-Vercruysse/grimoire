use leptos::{logging, server};
use server_fn::{
    ServerFnError,
    codec::{MultipartData, MultipartFormData},
};
use uuid::Uuid;

use crate::{
    core::types::FilePreview,
    features::storage::types::{FileStorage, NewFileRecord},
};

#[cfg(feature = "ssr")]
use crate::features::storage::server::*;

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
        let mut files = files.into_inner().unwrap();
        while let Ok(Some(field)) = files.next_field().await {
            let id = Uuid::new_v4();
            let filename = field.file_name().unwrap_or_default().to_string();
            let mime = field.content_type().unwrap().to_string();
            let content = field.bytes().await?.to_vec();
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
                    size: size as i32,
                    storage_path,
                };

                if let Err(_) = store_in_db(file).await {
                    return Err(ServerFnError::ServerError(
                        "Failed to store in db".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}
