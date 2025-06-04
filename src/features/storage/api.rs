use leptos::server;
use server_fn::ServerFnError;

use crate::core::types::FilePreview;

#[server]
pub async fn load_users_files() -> Result<Vec<FilePreview>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::server::get_files;
        let mut files: Vec<FilePreview> = Vec::new();
        if let Ok(records) = get_files().await {
            files = records
                .iter()
                .map(|file| FilePreview::from_record(file.clone()))
                .collect();
        }
        return Ok(files);
    }
    // Err(ServerFnError::ServerError("Not running on server".into()))
}
