use leptos::prelude::*;

use crate::core::config::UPLOAD_DIR;
use crate::features::upload::types::{FileRecord, NewFileRecord};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[server]
pub async fn save_uploaded_file(
    filename: String,
    bytes: Vec<u8>,
    id: Uuid,
) -> Result<String, ServerFnError> {
    let folder = format!("{}/user/{}", UPLOAD_DIR, id);
    fs::create_dir_all(&folder).await?;

    let full_path = format!("{}/{}", folder, filename);

    let mut file = File::create(&full_path).await?;
    file.write_all(&bytes).await?;

    Ok(full_path)
}

#[server]
pub async fn get_files() -> Result<Vec<FileRecord>, ServerFnError> {
    use self::ssr::db;

    let mut conn = db().await?;
    let files = sqlx::query_as!(FileRecord, "SELECT * from files")
        .fetch_all(&mut conn)
        .await?;

    Ok(files)
}

#[server]
pub async fn store_in_db(file: NewFileRecord) -> Result<(), ServerFnError> {
    use self::ssr::db;
    let mut conn = db().await?;

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(250));

    match sqlx::query(
        "INSERT INTO files (id, filename, mime_type, size, storage_path) VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(file.id)
    .bind(file.filename)
    .bind(file.mime_type)
    .bind(file.size as i32)
    .bind(file.storage_path)
    .execute(&mut conn)
    .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::new(e)),
    }
}

pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use dotenvy::dotenv;
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, PgConnection};
    use std::env;

    pub async fn db() -> Result<PgConnection, ServerFnError> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")?;
        Ok(PgConnection::connect(&db_url).await?)
    }
}
