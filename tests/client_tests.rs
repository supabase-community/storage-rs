use supabase_storage::models::{
    Column, DownloadOptions, FileSearchOptions, MimeType, Order, SortBy, StorageClient,
    TransformOptions,
};
use uuid::Uuid;

async fn create_test_client() -> StorageClient {
    StorageClient::new_from_env().await.unwrap()
}

#[tokio::test]
async fn test_create_client_from_env() {
    let client = StorageClient::new_from_env().await.unwrap();
    assert_eq!(std::env::var("SUPABASE_URL").unwrap(), client.project_url)
}

#[tokio::test]
async fn test_create_bucket() {
    let client = create_test_client().await;

    let name = client
        // NOTE: Intentionally leaving the ID blank will use the name as the id
        .create_bucket("a-cool-name-for-a-bucket", None, false, None, None)
        .await
        .unwrap();

    assert_eq!(name, "a-cool-name-for-a-bucket");

    // Delete first bucket
    client
        .delete_bucket("a-cool-name-for-a-bucket")
        .await
        .unwrap();

    // Create new bucket with UUID for id instead of name
    let bucket_id = Uuid::now_v7().to_string();
    let new_name = client
        // NOTE: Intentionally leaving the ID blank will use the name as the id
        .create_bucket(
            "a-totally-different-cool-name-for-a-bucket",
            Some(&bucket_id),
            false,
            None,
            None,
        )
        .await
        .unwrap();

    // Delete second bucket
    client.delete_bucket(&bucket_id).await.unwrap();

    assert_eq!("a-totally-different-cool-name-for-a-bucket", new_name);
}

#[tokio::test]
async fn test_create_bucket_with_options() {
    let client = create_test_client().await;

    let name = client
        // NOTE: Intentionally leaving the ID blank will use the name as the id
        .create_bucket(
            "a-cool-name-for-a-bucket-with-options",
            None,
            false,
            Some(vec![
                MimeType::WAV,
                MimeType::PNG,
                MimeType::Custom("image/*"),
            ]),
            Some(12431243),
        )
        .await
        .unwrap();

    // Get the bucket, and make sure the options are present
    let bucket = client
        .get_bucket("a-cool-name-for-a-bucket-with-options")
        .await
        .unwrap();

    assert!(bucket.allowed_mime_types.is_some() && 12431243 == bucket.file_size_limit);

    // Delete bucket
    client
        .delete_bucket("a-cool-name-for-a-bucket-with-options")
        .await
        .unwrap();

    assert_eq!(name, "a-cool-name-for-a-bucket-with-options");
}

#[tokio::test]
async fn test_list_buckets() {
    let client = create_test_client().await;

    // Add a bucket with options
    client
        .create_bucket(
            "test_bucket_for_list",
            Some("test_bucket_for_list"),
            false,
            Some(vec![MimeType::WAV, MimeType::PNG]),
            Some(0),
        )
        .await
        .unwrap();

    let buckets = client.list_buckets().await.unwrap();

    assert!(
        buckets
            .iter()
            .any(|bucket| bucket.name == "test_bucket_for_list"),
        "test_bucket_for_list should exist in buckets"
    );

    // Delete bucket
    client.delete_bucket("test_bucket_for_list").await.unwrap();
}

#[tokio::test]
async fn test_get_bucket() {
    let client = create_test_client().await;

    let bucket = client.get_bucket("example_private_bucket_1").await.unwrap();

    assert!(bucket.name == "example_private_bucket_1")
}
// DELETE
#[tokio::test]
async fn test_delete_bucket() {
    let client = create_test_client().await;

    let bucket = client
        .create_bucket("test_delete_bucket", None, false, None, None)
        .await
        .unwrap();

    assert_eq!(bucket, "test_delete_bucket");

    let delete = client.delete_bucket("test_delete_bucket").await;

    assert!(delete.is_ok())
}

#[tokio::test]
async fn test_update_bucket() {
    let client = create_test_client().await;

    client
        .create_bucket(
            "test_update_bucket",
            None,
            false, // make bucket private
            Some(vec![MimeType::WAV, MimeType::PNG, MimeType::Custom("")]),
            Some(12431243),
        )
        .await
        .unwrap();

    client
        .update_bucket("test_update_bucket", true, None, None) // make bucket public
        .await
        .unwrap();

    let bucket = client.get_bucket("test_update_bucket").await.unwrap();

    assert!(bucket.public);

    // Delete bucket
    let delete = client.delete_bucket("test_update_bucket").await;

    assert!(delete.is_ok());
}

