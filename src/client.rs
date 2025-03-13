use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE},
    Url,
};

use crate::{
    errors::Error,
    models::{
        Bucket, BucketResponse, Buckets, CopyFilePayload, CopyFileResponse, CreateBucket,
        CreateBucketResponse, CreateMultipleSignedUrlsPayload, CreateSignedUrlPayload,
        DownloadOptions, FileObject, FileOptions, FileSearchOptions, ListFilesPayload, MimeType,
        MoveFilePayload, ObjectResponse, SignedUploadUrlResponse, SignedUrlResponse, StorageClient,
        UpdateBucket, UploadToSignedUrlResponse, HEADER_API_KEY, STORAGE_V1,
    },
};

impl StorageClient {
    /// Create a new StorageClient from a project_url and api_key
    /// You can find your project url and keys at `https://supabase.com/dashboard/project/YOUR_PROJECT_ID/settings/api`
    /// # Example
    /// ```
    /// let client = StorageClient::new(project_url, api_key, jwt_secret).unwrap();
    /// ```
    pub fn new(project_url: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            project_url,
            api_key,
            headers: HeaderMap::new(),
        }
    }

    /// Create a new StorageClient from the "SUPABASE_URL" and "SUPABASE_API_KEY" environment
    /// variables.
    ///
    /// Create a new StorageClient from a project_url and api_key
    /// You can find your project url and keys at `https://supabase.com/dashboard/project/YOUR_PROJECT_ID/settings/api`
    /// # Example
    /// ```
    /// let client = StorageClient::new_from_env().unwrap();
    /// ```
    pub async fn new_from_env() -> Result<StorageClient, Error> {
        let project_url = std::env::var("SUPABASE_URL")?;
        let api_key = std::env::var("SUPABASE_API_KEY")?;

        Ok(StorageClient {
            client: reqwest::Client::new(),
            project_url,
            api_key,
            headers: HeaderMap::new(),
        })
    }

    pub fn insert_header(
        mut self,
        header_name: impl IntoHeaderName,
        header_value: impl AsRef<str>,
    ) -> Self {
        self.headers.insert(
            header_name,
            HeaderValue::from_str(header_value.as_ref()).expect("Invalid header value."),
        );
        self
    }

    /// Create a new storage bucket, returning the name **_(not the id)_** of the bucket on success.
    ///
    /// Requires your StorageClient to have the following RLS permissions:
    /// `buckets` table permissions: insert
    ///
    /// WARNING: Do not use underscores in bucket names or ids
    ///
    /// # Example
    ///
    ///```rust
    /// let name = client
    ///     .create_bucket("a-cool-name-for-a-bucket", None, false, None, None)
    ///     .await
    ///     .unwrap();
    ///```
    pub async fn create_bucket<'a>(
        &self,
        name: &str,
        id: Option<&str>,
        public: bool,
        allowed_mime_types: Option<Vec<MimeType<'a>>>,
        file_size_limit: Option<u64>,
    ) -> Result<String, Error> {
        let mut headers = self.headers.clone();
        headers.insert(HEADER_API_KEY, HeaderValue::from_str(&self.api_key)?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }
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
    ///
    /// # Example
    /// ```rust
    /// client.delete_bucket("a-cool-name-for-a-bucket").await.unwrap();
    /// ```
    pub async fn delete_bucket(&self, id: &str) -> Result<(), Error> {
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

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
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }
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
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }
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
    ///
    /// # Example
    /// ```rust
    /// client.update_bucket("bucket_id", true, None, Some(100_000_000)).await.unwrap();
    /// ```
    pub async fn update_bucket<'a>(
        &self,
        id: &str,
        public: bool,
        allowed_mime_types: Option<Vec<MimeType<'a>>>,
        file_size_limit: Option<u64>,
    ) -> Result<String, Error> {
        let mut headers = self.headers.clone();
        headers.insert(HEADER_API_KEY, HeaderValue::from_str(&self.api_key)?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }
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

    /// Empty a bucket with a given id
    /// # Example
    /// ```rust
    /// let empty = client.empty_bucket("empty_bucket_test").await.unwrap();
    /// ```
    pub async fn empty_bucket(&self, id: &str) -> Result<String, Error> {
        let mut headers = self.headers.clone();
        headers.insert(HEADER_API_KEY, HeaderValue::from_str(&self.api_key)?); // maybe delete
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

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
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

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

    /// Replaces the file at the designated bucket and path with the given `Vec<u8>`
    ///
    /// # Example
    /// ```rust
    /// let object = client.replace_file("bucket_id", file, "path/to/file.txt", Some(options)).await.unwrap();
    /// ```
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

    /// Updates the file at the designated bucket and path with the given `Vec<u8>`
    ///
    /// This is identical to `replace_file`
    ///
    /// # Example
    /// ```rust
    /// let object = client.update_file("bucket_id", file, "path/to/file.txt", Some(options)).await.unwrap();
    /// ```
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

    /// Uploads a file at the designated bucket and path with the given `Vec<u8>`
    ///
    /// # Example
    /// ```rust
    /// let object = client.upload_file("bucket_id", file, "path/to/file.txt", Some(options)).await.unwrap();
    /// ```
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

    // TODO: Incorporate download options
    /// Download the designated file
    /// # Example
    /// ```rust
    /// let file = client.download_file("bucket_id", file, "path/to/file.txt", Some(options)).await.unwrap();
    /// ```
    pub async fn download_file(
        &self,
        bucket_id: &str,
        path: &str,
        options: Option<DownloadOptions<'_>>,
    ) -> Result<Vec<u8>, Error> {
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

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

    /// Delete the designated file, returning a confirmation message on success
    ///
    ///```rust
    /// let message = client
    ///     .delete_file("upload_tests", "tests/signed_upload")
    ///     .await
    ///     .unwrap();
    ///```
    pub async fn delete_file(&self, bucket_id: &str, path: &str) -> Result<BucketResponse, Error> {
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

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

    /// List all files that match your search criteria
    ///
    /// The returned `Vec<FileObject>` will contain both files and folders. Folders can be
    /// identified as having a populated name field, without any other fields.
    ///
    /// # Example
    /// ```rust
    ///
    /// let options = FileSearchOptions {
    ///     limit: Some(5), // List up to five files
    ///     offset: Some(1), // Skip the first file
    ///     sort_by: Some(SortBy {
    ///         column: Column::Name, // Sort by name
    ///         order: Order::Asc, // In Ascending order
    ///     }),
    ///     search: None, // With no specific search string
    /// };
    ///
    /// client
    ///     .list_files("bucket_id", "folder/", Some(options))
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn list_files(
        &self,
        bucket_id: &str,
        path: Option<&str>,
        options: Option<FileSearchOptions<'_>>,
    ) -> Result<Vec<FileObject>, Error> {
        let mut headers = self.headers.clone();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

        let options = options.unwrap_or_default();
        let payload = ListFilesPayload {
            limit: options.limit,
            offset: options.offset,
            sort_by: options.sort_by,
            prefix: path.unwrap_or(""),
            search: options.search,
        };

        let body = serde_json::to_string(&payload).unwrap();

        let res = self
            .client
            .post(format!(
                "{}{}/object/list/{}",
                self.project_url, STORAGE_V1, bucket_id,
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let files: Vec<FileObject> =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(files)
    }

    /// Copy a file from one path to another
    /// # Example
    ///
    /// ```rust
    /// // Copies `3.txt` into `folder/4.txt` within the same bucket, including metadata
    /// let key = client
    ///     .copy_file("from_bucket", None, "3.txt", Some("folder/4.txt"), true)
    ///     .await
    ///     .unwrap();
    ///
    /// // Copies `a.txt` into `folder/b.txt` in a different bucket, including metadata
    /// let key = client
    ///     .copy_file("from_bucket", "to_bucket", "a.txt", Some("folder/b.txt"), true)
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn copy_file(
        &self,
        from_bucket: &str,
        to_bucket: Option<&str>,
        from_path: &str,
        to_path: Option<&str>,
        copy_metadata: bool,
    ) -> Result<String, Error> {
        let mut headers = self.headers.clone();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

        let payload = CopyFilePayload {
            bucket_id: from_bucket,
            source_key: from_path,
            destination_bucket: to_bucket.unwrap_or(from_bucket),
            destination_key: to_path.unwrap_or(from_path),
            copy_metadata,
        };

        let body = serde_json::to_string(&payload)?;

        let res = self
            .client
            .post(format!("{}{}/object/copy", self.project_url, STORAGE_V1,))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let value: CopyFileResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(value.key)
    }

    /// Create a signed download url, returns a signed_url on success
    ///
    /// # Example
    /// ```rust
    ///
    /// client
    ///    .create_signed_url("list_files", "3.txt", 12431234)
    ///    .await
    ///    .unwrap();
    /// ```
    pub async fn create_signed_url(
        &self,
        bucket_id: &str,
        path: &str,
        expires_in: u64,
        options: Option<DownloadOptions<'_>>,
    ) -> Result<String, Error> {
        let mut headers = self.headers.clone();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

        let payload = CreateSignedUrlPayload {
            expires_in,
            transform: options.and_then(|opts| opts.transform),
        };

        let body = serde_json::to_string(&payload)?;

        let res = self
            .client
            .post(format!(
                "{}{}/object/sign/{}/{}",
                self.project_url, STORAGE_V1, bucket_id, path
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let signed_url_response: SignedUrlResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(format!(
            "{}{}{}",
            self.project_url, STORAGE_V1, signed_url_response.signed_url
        ))
    }

    /// Create multiple signed download urls, returns a `Vec` of signed_urls on success
    ///
    /// # Example
    /// ```rust
    ///
    /// let urls = client
    ///    .create_multiple_signed_urls("bucket_id", vec!["1.txt", "2.txt", "3.txt"], 100_000)
    ///    .await
    ///    .unwrap();
    /// ```
    pub async fn create_multiple_signed_urls(
        &self,
        bucket_id: &str,
        paths: Vec<&str>,
        expires_in: u64,
    ) -> Result<Vec<String>, Error> {
        let mut headers = self.headers.clone();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

        let payload = CreateMultipleSignedUrlsPayload { expires_in, paths };

        let body = serde_json::to_string(&payload)?;

        let res = self
            .client
            .post(format!(
                "{}{}/object/sign/{}",
                self.project_url, STORAGE_V1, bucket_id
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let signed_url_response: Vec<SignedUrlResponse> =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        let signed_urls: Vec<String> = signed_url_response
            .into_iter()
            .map(|r| format!("{}{}{}", self.project_url, STORAGE_V1, r.signed_url))
            .collect();

        Ok(signed_urls)
    }

    /// Create a signed upload url,
    ///
    /// Returns the `url` (without hostname) and authorization `token` on success
    ///
    /// # Example
    /// ```rust
    /// let signed = client.create_signed_upload_url("list_files", "42.txt").await.unwrap();
    /// ```
    pub async fn create_signed_upload_url(
        &self,
        bucket_id: &str,
        path: &str,
    ) -> Result<SignedUploadUrlResponse, Error> {
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

        let res = self
            .client
            .post(format!(
                "{}{}/object/upload/sign/{}/{}",
                self.project_url, STORAGE_V1, bucket_id, path
            ))
            .headers(headers)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let response: SignedUploadUrlResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(response)
    }

    /// Upload a file to a signed url
    ///
    /// Returns the `url` (without hostname) and authorization `token` upon success
    ///
    /// # Example
    /// ```rust
    /// let object = client
    ///     .upload_to_signed_url("bucket_id", "upload_token", file, "path/to/file.txt", None).await.unwrap();
    ///```
    pub async fn upload_to_signed_url(
        &self,
        bucket_id: &str,
        token: &str,
        data: Vec<u8>,
        path: &str,
        options: Option<FileOptions<'_>>,
    ) -> Result<UploadToSignedUrlResponse, Error> {
        let mut headers = self.headers.clone();
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

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

        let res = self
            .client
            .put(format!(
                "{}{}/object/upload/sign/{}/{}?token={}",
                self.project_url, STORAGE_V1, bucket_id, path, token
            ))
            .headers(headers)
            .body(data)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let response: UploadToSignedUrlResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(response)
    }

    /// Returns a public URL for accessing an asset in a storage bucket
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - Unique identifier for the storage bucket
    /// * `path` - Path to the file within the bucket, formatted as 'folder/subfolder/filename.ext'
    /// * `options` - Optional parameters for customizing the download URL:
    ///   - Image transformations (uses `/render/image` endpoint)
    ///   - Download behavior configurations
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Basic usage
    /// let url = client.get_public_url("photos", "vacations/beach.jpg", None).await?;
    ///
    /// // With image transformation
    /// let options = DownloadOptions {
    ///     transform: Some(Transform { width: 300, ..Default::default() }),
    ///     ..Default::default()
    /// };
    /// let url = client.get_public_url("photos", "vacations/beach.jpg", Some(options)).await?;
    /// ```
    ///
    /// # Note
    ///
    /// The URL can also be manually constructed by combining:
    /// `{project_url}/storage/v1/object/public/{bucket_id}/{path}`
    pub async fn get_public_url(
        &self,
        bucket_id: &str,
        path: &str,
        options: Option<DownloadOptions<'_>>,
    ) -> Result<String, Error> {
        let renderpath = match &options {
            Some(opts) if opts.transform.is_some() => "render/image",
            _ => "object",
        };

        let url_str = format!(
            "{}{STORAGE_V1}/{renderpath}/public/{bucket_id}/{path}",
            self.project_url
        );

        match options {
            Some(opts) => build_url_with_options(&url_str, &opts),
            None => Ok(url_str),
        }
    }

    /// Move a file from one path to another
    /// # Example
    ///
    /// ```rust
    /// // Copies `3.txt` into `folder/4.txt` within the same bucket
    /// let key = client
    ///     .move_file("from_bucket", None, "3.txt", Some("folder/4.txt"))
    ///     .await
    ///     .unwrap();
    ///
    /// // Copies `a.txt` into `folder/b.txt` in a different bucket
    /// let key = client
    ///     .move_file("from_bucket", "to_bucket", "a.txt", Some("folder/b.txt"))
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn move_file(
        &self,
        from_bucket: &str,
        to_bucket: Option<&str>,
        from_path: &str,
        to_path: &str,
    ) -> Result<String, Error> {
        let mut headers = self.headers.clone();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        if !headers.contains_key(AUTHORIZATION) {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
            );
        }

        let payload = MoveFilePayload {
            bucket_id: from_bucket,
            source_key: from_path,
            destination_bucket: to_bucket.unwrap_or(from_bucket),
            destination_key: to_path,
        };

        let body = serde_json::to_string(&payload)?;

        let res = self
            .client
            .post(format!("{}{}/object/move", self.project_url, STORAGE_V1,))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = res.status();
        let res_body = res.text().await?;

        let value: BucketResponse =
            serde_json::from_str(&res_body).map_err(|_| Error::StorageError {
                status: res_status,
                message: res_body,
            })?;

        Ok(value.message)
    }
}

pub fn build_url_with_options(url_str: &str, options: &DownloadOptions) -> Result<String, Error> {
    let mut url = Url::parse(url_str).map_err(|_| Error::UrlParseError {
        message: "Failed to parse Url".to_string(),
    })?;

    let mut query_pairs = url.query_pairs_mut();

    if let Some(transform) = &options.transform {
        if let Some(height) = transform.height {
            query_pairs.append_pair("height", &height.to_string());
        }

        if let Some(width) = transform.width {
            query_pairs.append_pair("width", &width.to_string());
        }

        if let Some(format) = &transform.format {
            query_pairs.append_pair("format", format);
        }

        if let Some(quality) = transform.quality {
            query_pairs.append_pair("quality", &quality.to_string());
        }

        if let Some(resize) = transform.resize {
            match resize {
                "conver" | "contain" | "fill" => {
                    query_pairs.append_pair("resize", resize);
                }
                _ => {} // Invalid resize option, ignore
            }
        }
    }

    // Use more descriptive parameter name
    if options.download.unwrap_or(false) {
        query_pairs.append_pair("download", "true");
    }

    // Release the mutable borrow before generating the final URL
    drop(query_pairs);

    Ok(url.to_string())
}

pub fn extract_token(url: &str) -> Result<&str, Error> {
    url.split('?')
        .nth(1)
        .and_then(|query| query.split('&').find(|param| param.starts_with("token=")))
        .and_then(|token_param| token_param.strip_prefix("token="))
        .ok_or_else(|| Error::InvalidToken {
            message: "No token found in URL".to_string(),
        })
}
