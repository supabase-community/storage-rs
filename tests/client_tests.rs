use supabase_storage::models::{MimeType, StorageClient};
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
            Some(vec![MimeType::WAV.into(), MimeType::PNG.into()]),
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

