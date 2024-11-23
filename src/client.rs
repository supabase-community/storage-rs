use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE},
    Url,
};

use crate::{
    errors::Error,
    models::{
        Bucket, BucketResponse, Buckets, CreateBucket, CreateBucketResponse, DownloadOptions,
        FileObject, FileOptions, FileSearchOptions, ListFilesPayload, MimeType, ObjectResponse,
        StorageClient, UpdateBucket, HEADER_API_KEY, STORAGE_V1,
    },
};

impl StorageClient {
    /// Create a new StorageClient from a project_url and api_key
    pub fn new(project_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            project_url,
            api_key,
        }
    }

    /// Create a new StorageClient from the "SUPABASE_URL" and "SUPABASE_API_KEY" environment
    /// variables.
    pub async fn new_from_env() -> Result<StorageClient, Error> {
        let project_url = std::env::var("SUPABASE_URL")?;
        let api_key = std::env::var("SUPABASE_API_KEY")?;

        Ok(StorageClient {
            client: reqwest::Client::new(),
            project_url,
            api_key,
        })
    }

    /// Create a new storage bucket, returning the name **_(not the id)_** of the bucket on success.
    ///
    /// Requires your StorageClient to have the following RLS permissions:
    /// `buckets` table permissions: insert
    ///
    /// WARNING: Do not use underscores in bucket names or ids
    pub async fn create_bucket<'a>(
        &self,
        name: &str,
        id: Option<&str>,
        public: bool,
        allowed_mime_types: Option<Vec<MimeType<'a>>>,
        file_size_limit: Option<u64>,
    ) -> Result<String, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_API_KEY, HeaderValue::from_str(&self.api_key)?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        // Convert MimeType enums to their string representations
        let mime_types: Option<Vec<String>> =
            allowed_mime_types.map(|types| types.iter().map(|mime| mime.to_string()).collect());

        let payload = CreateBucket {
            id: Some(id.map(Into::into).unwrap_or_else(|| name)),
            name,
            public,
            allowed_mime_types: mime_types,
            file_size_limit,
        };

        let request_body = serde_json::to_string(&payload)?;

        let res = self
            .client
            .post(format!("{}{}/bucket", self.project_url, STORAGE_V1))
            .headers(headers)
            .body(request_body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let bucket: CreateBucketResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(bucket.name)
    }

    /// Delete the bucket with the given id
    pub async fn delete_bucket(&self, id: &str) -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );

        let res = self
            .client
            .delete(format!("{}{}/bucket/{}", self.project_url, STORAGE_V1, id))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        if res_status.is_success() {
            return Ok(());
        } else {
            return Err(Error::StorageError {
                status: res_status,
                message: res_body,
            });
        }
    }

    /// Get the bucket with the given id
    /// # Example
    /// ```
    /// let bucket = client
    ///     .get_bucket("a-cool-name-for-a-bucket-with-options")
    ///     .await
    ///     .unwrap();
    ///```
    pub async fn get_bucket(&self, bucket_id: &str) -> Result<Bucket, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let res = self
            .client
            .get(format!(
                "{}{}/bucket/{}",
                self.project_url, STORAGE_V1, bucket_id
            ))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let bucket: Bucket = serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
            status: res_status,
            message: res_body,
        })?;

        Ok(bucket)
    }

    /// Retrieves the details of all Storage buckets within an existing project
    /// # Example
    /// ```
    /// let buckets = client.list_buckets().await.unwrap();
    /// ```
    pub async fn list_buckets(&self) -> Result<Buckets, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let res = self
            .client
            .get(format!("{}{}/bucket", self.project_url, STORAGE_V1))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let buckets = serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
            status: res_status,
            message: res_body,
        })?;

        Ok(buckets)
    }

    /// Updates a Storage bucket
    ///
    /// Requires the following RLS permissions:
    /// `buckets` table: `select` and `update`
    pub async fn update_bucket<'a>(
        &self,
        id: &str,
        public: bool,
        allowed_mime_types: Option<Vec<MimeType<'a>>>,
        file_size_limit: Option<u64>,
    ) -> Result<String, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_API_KEY, HeaderValue::from_str(&self.api_key)?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        // Convert MimeType enums to their string representations
        let mime_types: Option<Vec<String>> =
            allowed_mime_types.map(|types| types.iter().map(|mime| mime.to_string()).collect());

        let payload = UpdateBucket {
            id,
            public,
            allowed_mime_types: mime_types,
            file_size_limit,
        };

        let request_body = serde_json::to_string(&payload)?;

        let res = self
            .client
            .put(format!("{}{}/bucket/{}", self.project_url, STORAGE_V1, id))
            .headers(headers)
            .body(request_body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let bucket: BucketResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(bucket.message)
    }

    /// Empty a bucket
    pub async fn empty_bucket(&self, id: &str) -> Result<String, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_API_KEY, HeaderValue::from_str(&self.api_key)?); // maybe delete
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );

        let res = self
            .client
            .post(format!(
                "{}{}/bucket/{}/empty",
                self.project_url, STORAGE_V1, id
            ))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let bucket: BucketResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(bucket.message)
    }

    async fn upload_or_update_file(
        &self,
        bucket_id: &str,
        data: Vec<u8>,
        path: &str,
        update: bool,
        options: Option<FileOptions<'_>>,
    ) -> Result<ObjectResponse, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );

        // Set optional headers
        if let Some(opts) = options {
            if let Some(cache_control) = opts.cache_control {
                headers.insert(
                    CACHE_CONTROL,
                    HeaderValue::from_str(&format!("{}", cache_control.as_secs()))?,
                );
            }

            if let Some(content_type) = opts.content_type {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_str(&format!("{}", content_type))?,
                );
            }

            if opts.upsert {
                headers.insert(
                    "x-upsert",
                    HeaderValue::from_str(&format!("{}", opts.upsert))?,
                );
            }
        }

        let res = match update {
            true => {
                self.client
                    .put(format!(
                        "{}{}/object/{}/{}",
                        self.project_url, STORAGE_V1, bucket_id, path
                    ))
                    .headers(headers)
                    .body(data)
                    .send()
                    .await?
            }
            false => {
                self.client
                    .post(format!(
                        "{}{}/object/{}/{}",
                        self.project_url, STORAGE_V1, bucket_id, path
                    ))
                    .headers(headers)
                    .body(data)
                    .send()
                    .await?
            }
        };

        let res_status = res.status();
        let res_body = res.text().await?;

        let object: ObjectResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(object)
    }

    pub async fn replace_file(
        &self,
        bucket_id: &str,
        data: Vec<u8>,
        path: &str,
        options: Option<FileOptions<'_>>,
    ) -> Result<ObjectResponse, Error> {
        self.upload_or_update_file(bucket_id, data, path, true, options)
            .await
    }

    pub async fn update_file(
        &self,
        bucket_id: &str,
        data: Vec<u8>,
        path: &str,
        options: Option<FileOptions<'_>>,
    ) -> Result<ObjectResponse, Error> {
        self.upload_or_update_file(bucket_id, data, path, true, options)
            .await
    }

    pub async fn upload_file(
        &self,
        bucket_id: &str,
        data: Vec<u8>,
        path: &str,
        options: Option<FileOptions<'_>>,
    ) -> Result<ObjectResponse, Error> {
        self.upload_or_update_file(bucket_id, data, path, false, options)
            .await
    }

    pub async fn download_file(
        &self,
        bucket_id: &str,
        path: &str,
        options: Option<DownloadOptions<'_>>,
    ) -> Result<Vec<u8>, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );

        let mut renderpath = "object";
        if let Some(opts) = options {
            if opts.transform.is_some() {
                renderpath = "render/image/authenticated"
            }
        }

        let res = self
            .client
            .get(format!(
                "{}{}/{}/{}/{}",
                self.project_url, STORAGE_V1, renderpath, bucket_id, path
            ))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.bytes().await?.to_vec();

        if !res_status.is_success() {
            return Err(Error::StorageError {
                status: res_status,
                message: String::from_utf8_lossy(&res_body).to_string(),
            });
        }

        Ok(res_body)
    }

    pub async fn delete_file(&self, bucket_id: &str, path: &str) -> Result<BucketResponse, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );

        let res = self
            .client
            .delete(format!(
                "{}{}/object/{}/{}",
                self.project_url, STORAGE_V1, bucket_id, path
            ))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let message: BucketResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(message)
    }
}
