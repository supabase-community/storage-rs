use reqwest::header::HeaderMap;

use crate::{errors::Error, models::StorageClient};

impl StorageClient {
    pub fn new(project_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            project_url,
            api_key,
            headers: HeaderMap::new(),
        }
    }

    pub fn new_from_env() -> Result<StorageClient, Error> {
        let project_url = std::env::var("SUPABASE_URL")?;
        let api_key = std::env::var("SUPABASE_API_KEY")?;

        Ok(StorageClient {
            client: reqwest::Client::new(),
            project_url,
            api_key,
            headers: HeaderMap::new(),
        })
    }
}
