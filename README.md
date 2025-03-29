# Supabase Storage

![Crates.io License](https://img.shields.io/crates/l/supabase-storage-rs?style=for-the-badge)
[![Crates.io Version](https://img.shields.io/crates/v/supabase-storage-rs?style=for-the-badge)](https://crates.io/crates/supabase-storage-rs)
[![docs.rs](https://img.shields.io/docsrs/supabase-storage-rs?style=for-the-badge)](https://docs.rs/supabase-storage-rs/latest/supabase_storage_rs/index.html) 

This is a Rust implementation of the [supabase storage client](https://supabase.com/docs/reference/javascript/storage-createbucket). The goal is to have feature parity and an easy-to-use API.

Currently this software is functional, but not yet battle-tested.

## Installation

### Cargo

```bash
cargo add supabase-storage-rs
```

## Usage

### Create a Storage Client
```rust
// You can manually pass in the values
let auth_client = StorageClient::new(project_url, api_key).unwrap();

// Or you can use environment variables
// Requires `SUPABASE_URL` and`SUPABASE_API_KEY` environment variables
let auth_client = StorageClient::new_from_env().unwrap();
```

### Create a Bucket

Create a new storage bucket. Returns the bucket name (not ID) on success.

```rust
let name = client
   .create_bucket(
       "a-cool-name-for-a-bucket",
       None,    // Optional bucket ID
       false,   // Public bucket
       None,    // Allowed MIME types
       None     // File size limit
   )
   .await
   .unwrap();
```

### Delete a Bucket

```rust
client.delete_bucket("a-cool-name-for-a-bucket").await.unwrap();
```

### Update a Bucket

```rust
client
   .update_bucket(
       "bucket_id",
       true,  // Make bucket public
       None,  // Keep existing MIME types
       None   // Keep existing size limit
   )
   .await
   .unwrap();
```

### Get a Bucket

```rust
let bucket = client.get_bucket("a-cool-name-for-a-bucket-with-options").await.unwrap();

// Returns a `Bucket`
pub struct Bucket {
    pub id: String,              // Bucket ID
    pub name: String,            // Bucket name
    pub owner: String,           // Owner's ID
    pub public: bool,            // Public/private status
    pub file_size_limit: Option<i64>,         // Max file size in bytes
    pub allowed_mime_types: Option<Vec<String>>, // Allowed file types
    pub created_at: String,      // Creation timestamp
    pub updated_at: String,      // Last update timestamp
}
```

### List Files in a Bucket

```rust
let options = FileSearchOptions {
   limit: Some(5),     // List up to five files
   offset: Some(1),    // Skip the first file
   sort_by: Some(SortBy {
       column: Column::Name,  // Sort by name
       order: Order::Asc,    // In ascending order
   }),
   search: None,       // No search string
};

client
   .list_files(
       "bucket_id", 
       "folder/",   // Path prefix to search
       Some(options) 
   )
   .await
   .unwrap();
```

### List Buckets

```rust
let buckets = client.list_buckets().await.unwrap();
```

### Empty a Bucket

```rust
let empty = client.empty_bucket("bucket_id").await.unwrap();
```

### Upload a File

```rust
let object = client
   .upload_file(
       "bucket_id",
       file,               // File data to upload
       "path/to/file.txt", // Destination path
       Some(options)       // Upload options
   )
   .await
   .unwrap();
```

### Update a File

```rust
let object = client
   .update_file(
       "bucket_id",
       file,               // File data
       "path/to/file.txt", // File path
       Some(options)       // File options
   )
   .await
   .unwrap();
```

### Download a File

```rust
let file = client
   .download_file(
       "bucket_id",
       "path/to/file.txt", // File path
       Some(options)       // Download options
   )
   .await
   .unwrap();
```

### Copy a File

```rust
// Copy within same bucket, including metadata
let key = client
   .copy_file(
       "from_bucket",
       None,                  // Same bucket
       "3.txt",               // Source path
       Some("folder/4.txt"),  // Destination path
       true                   // Copy metadata
   )
   .await
   .unwrap();

// Copy between different buckets
let key = client
   .copy_file(
       "from_bucket",
       Some("to_bucket"),    // Destination bucket
       "a.txt",              // Source path
       Some("folder/b.txt"), // Destination path
       true                  // Copy metadata
   )
   .await
   .unwrap();
```

### Delete a File

```rust
let message = client
   .delete_file(
       "upload_tests",         // Bucket ID
       "tests/signed_upload"   // File path
   )
   .await
   .unwrap();
```

### Create a Signed URL

```rust
let signed_url = client
   .create_signed_url(
       "bucket_id",        // Bucket ID
       "folder/file.txt",  // File path
       12431234            // Expiry time in seconds
   )
   .await
   .unwrap();
```

### Create Multiple Signed URLs

```rust
let urls = client
   .create_multiple_signed_urls(
       "bucket_id",
       vec!["1.txt", "2.txt", "3.txt"], // File paths
       100_000                          // Expiry time in seconds
   )
   .await
   .unwrap();
```

### Create a Signed Upload URL

Returns SignedUploadUrlResponse containing:

`url`: Path without hostname
`token`: Authorization token

```rust
let signed = client
   .create_signed_upload_url(
       "list_files",  // Bucket ID
       "42.txt"       // File path
   )
   .await
   .unwrap();
```

### Upload to a Signed URL

```rust
let object = client
   .upload_to_signed_url(
       "bucket_id",
       "upload_token",      // Token from SignedUploadUrlResponse
       file,                // File data as Vec<u8>
       "path/to/file.txt",  // File path
       None                 // File options
   )
   .await
   .unwrap();
```

### Get Public URL

```rust
// Basic usage
let url = client
   .get_public_url(
       "photos",                 // Bucket ID
       "vacations/beach.jpg",    // File path
       None                      // No options
   )
   .await
   .unwrap();

// With image transformation
let options = DownloadOptions {
   transform: Some(Transform { 
       width: 300,
       ..Default::default() 
   }),
   download: Some(true)
};

let url = client
   .get_public_url(
       "photos",
       "vacations/beach.jpg",
       Some(options)
   )
   .await
   .unwrap();
```

## Features
- [x] Create Bucket 
- [x] Delete Bucket 
- [x] Update Buckets
- [x] Get Bucket 
- [x] List Buckets
- [x] Empty Bucket
- [x] Upload a file 
- [x] Update a file 
- [x] Download a file 
- [x] List files in a bucket 
- [x] Replace a file in a bucket
- [x] Move a file in a bucket
- [x] Copy a file in a bucket
- [x] Delete files in a bucket
- [x] Create signed URL 
- [x] Create multiple signed URLs
- [x] Create signed upload URLs
- [x] Upload to a signed URL
- [x] Retrieve public URL


## Contributions

Contributors are always welcome. I only ask that you add or update tests to cover your changes. Until this crate reaches 1.0.0 we're in the "move fast and break things" phase. Don't concern yourself with elegance.
