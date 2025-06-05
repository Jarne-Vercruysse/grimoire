use leptos::{logging, prelude::*};

use super::types::{FileRecord, FileStorage, NewFileRecord};
use crate::core::config::UPLOAD_DIR;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[server]
pub async fn save_uploaded_file(file: FileStorage) -> Result<String, ServerFnError> {
    let folder = format!("{}/user/{}", UPLOAD_DIR, file.id);
    fs::create_dir_all(&folder).await?;

    let full_path = format!("{}/{}", folder, file.filename);

    let mut blob = File::create(&full_path).await?;
    blob.write_all(&file.content).await?;

    Ok(full_path)
}

#[server]
pub async fn get_files() -> Result<Vec<FileRecord>, ServerFnError> {
    use self::ssr::db;

    // fake a slower load of all the files
    //std::thread::sleep(std::time::Duration::from_millis(2500));
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
    //std::thread::sleep(std::time::Duration::from_millis(2500));

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
        Ok(_row) => {
            logging::log!("Stored in DB");
            Ok(())
        }
        Err(e) => Err(ServerFnError::new(e)),
    }
}

#[server]
pub async fn get_file(id: Uuid) -> Result<FileRecord, ServerFnError> {
    use self::ssr::db;

    let mut conn = db().await?;
    let file = sqlx::query_as!(FileRecord, "SELECT * from files WHERE id = $1", id)
        .fetch_one(&mut conn)
        .await?;

    Ok(file)
}

#[server]
pub async fn delete_file_in_storage(id: Uuid) -> Result<(), ServerFnError> {
    let folder = format!("{}/user/{}", UPLOAD_DIR, id);
    fs::remove_dir_all(folder).await?;
    Ok(())
}

#[server]
pub async fn get_stored_file(id: Uuid, filename: String) -> Result<FileStorage, ServerFnError> {
    let folder = format!("{}/user/{}/{}", UPLOAD_DIR, id, filename);
    let content = fs::read(folder).await?;
    let file = FileStorage {
        id,
        filename,
        content,
    };

    Ok(file)
}

#[server]
pub async fn delete_file_in_db(id: Uuid) -> Result<(), ServerFnError> {
    use self::ssr::db;

    let mut conn = db().await?;
    let _ = sqlx::query("DELETE FROM files WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await?;

    Ok(())
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
