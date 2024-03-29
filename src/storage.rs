use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::ErrorKind;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Article {
    pub title: String,
    pub body: String,
    pub creation_timestamp: i64,
    pub last_edit_timestamp: Option<i64>,
    pub edits: usize,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Storage {
    pub articles: Vec<Article>,
}

const STORAGE_FILE_NAME: &str = "nanowiki_storage.bin";

impl Storage {
    pub async fn read() -> Result<Self, Box<dyn Error>> {
        let bytes = match tokio::fs::read(STORAGE_FILE_NAME).await {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let mut f = File::create(STORAGE_FILE_NAME).await?;
                f.write_all(&rmp_serde::to_vec(&Self::default())?).await?;
                return Ok(Self::default());
            }
            Err(err) => return Err(Box::new(err)),
        };
        let storage = rmp_serde::from_slice(&bytes)?;

        Ok(storage)
    }

    pub async fn write(&self) -> Result<(), Box<dyn Error>> {
        let bytes = rmp_serde::to_vec(self)?;
        let mut f = File::create(STORAGE_FILE_NAME).await?;
        f.write_all(&bytes).await?;

        Ok(())
    }
}
