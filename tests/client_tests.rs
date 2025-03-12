use supabase_storage_rs::models::{
    Column, FileSearchOptions, MimeType, Order, SortBy, StorageClient,

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

    let bucket_name = client
        // NOTE: Intentionally leaving the ID blank will use the name as the id
        .create_bucket("a-cool-name-for-a-bucket", None, false, None, None)
        .await
        .unwrap();

    assert_eq!(bucket_name, "a-cool-name-for-a-bucket");

    // Delete first bucket
    client
        .delete_bucket("a-cool-name-for-a-bucket")
        .await
        .unwrap();

    // Create new bucket with UUID for id instead of name
    let bucket_id = Uuid::now_v7().to_string();
    let new_name = client
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

    assert!(bucket.allowed_mime_types.is_some() && 12431243 == bucket.file_size_limit.unwrap());

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

#[tokio::test]
async fn test_empty_bucket() {
    let client = create_test_client().await;

    // Add file to bucket
    let bytes = "byte array".as_bytes().to_vec();

    let _upload = client
        .upload_file("empty_bucket_test", bytes, "empty_test", None)
        .await;

    // Empty the bucket
    let empty = client.empty_bucket("empty_bucket_test").await.unwrap();

    assert!(empty == "Successfully emptied")
}

#[tokio::test]
async fn test_upload_file() {
    let client = create_test_client().await;

    let bytes = "byte array".as_bytes().to_vec();

    let upload = client
        .upload_file("upload_tests", bytes, "tests/Upload", None)
        .await;

    assert!(upload.is_ok());

    client
        .delete_file("upload_tests", "tests/Upload")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_upload_to_signed_url() {
    let client = create_test_client().await;

    let bytes = "byte array".as_bytes().to_vec();

    let upload = client
        .create_signed_upload_url("upload_tests", "tests/signed_upload")
        .await
        .unwrap();

    client
        .upload_to_signed_url(
            "upload_tests",
            &upload.token,
            bytes,
            "tests/signed_upload",
            None,
        )
        .await
        .unwrap();

    client
        .delete_file("upload_tests", "tests/signed_upload")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_update_file() {
    let client = create_test_client().await;

    let bytes = "updated byte array".as_bytes().to_vec();
    let new_bytes = "updated byte array".as_bytes().to_vec();

    let _upload = client
        .upload_file("upload_tests", bytes.clone(), "tests/Update", None)
        .await
        .unwrap();

    let file = client
        .download_file("upload_tests", "tests/Update", None)
        .await
        .unwrap();

    assert_eq!(file, bytes);

    let _update = client
        .update_file("upload_tests", new_bytes.clone(), "/tests/Update", None)
        .await
        .unwrap();

    let file = client
        .download_file("upload_tests", "tests/Update", None)
        .await
        .unwrap();

    assert_eq!(file, new_bytes);

    client
        .delete_file("upload_tests", "tests/Update")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_list_files() {
    let client = create_test_client().await;

    let options = FileSearchOptions {
        limit: Some(5),
        offset: Some(1),
        sort_by: Some(SortBy {
            column: Column::Name,
            order: Order::Asc,
        }),
        search: None,
    };

    // Contains folders and files
    let root_files = client
        .list_files("list_files", None, Some(options.clone()))
        .await
        .unwrap();

    assert!(root_files.len() >= 3);

    // Contains empty folders
    let folder_of_folders = client
        .list_files("list_files", Some("folder_of_folders"), None)
        .await
        .unwrap();

    assert!(folder_of_folders.len() > 0);

    // Contains nothing
    let empty_folder = client
        .list_files("list_files", Some("inner_folder"), None)
        .await
        .unwrap();

    assert!(empty_folder.len() == 0);
}

#[tokio::test]
async fn test_download_file() {
    let client = create_test_client().await;

    // NOTE: Disabled due to requiring a Pro plan for transformations
    //
    // let options = DownloadOptions {
    //     transform: Some(TransformOptions {
    //         width: Some(100),
    //         height: Some(300),
    //         resize: Some("conver"),
    //         format: None,
    //         quality: Some(80),
    //     }),
    //     download: None,
    // };

    client
        .download_file("list_files", "/folder/aaa.jpg", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_file() {
    let client = create_test_client().await;

    let key = client
        .copy_file("list_files", None, "3.txt", Some("folder/4.txt"), true)
        .await
        .unwrap();

    assert!(key == "list_files/folder/4.txt");

    client
        .delete_file("list_files", "folder/4.txt")
        .await
        .unwrap();
}

// #[tokio::test]
// async fn test_create_signed_url() {
//     let client = create_test_client().await;

    
//     let signed_url = client
//         .create_signed_url(&bucket_name, &upload_path, 2000, None)
//         .await
//         .unwrap();

//     assert!(signed_url.contains(&format!("/object/sign/{}/{}", bucket_name, upload_path)));
// }

// #[tokio::test]
// async fn test_create_signed_url_with_transform() {
//     let client = create_test_client().await;
//     // let (upload_path, file, bucket_name) = setup_test(&client).await;

//     client.upload_file(&bucket_name, file, &upload_path, None).await.expect("expected file to be uploaded");

//     let signed_url = client
//         .create_signed_url(&bucket_name, &upload_path, 2000, Some(DownloadOptions{
//             transform: Some(TransformOptions {
//                 width:Some(100),
//                 height:Some(100), 
//                 resize:None,
//                 format:None,
//                 quality:None,
//             }),
//             download: Some(false),
//         }))
//         .await
//         .unwrap();

//     assert!(signed_url.contains(&format!("/render/image/sign/{}/{}", bucket_name, upload_path)));
// }

#[tokio::test]
async fn test_create_signed_upload_url() {
    let client = create_test_client().await;

    client
        .create_signed_upload_url("list_files", "42.txt")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_multiple_signed_urls() {
    let client = create_test_client().await;

    let urls = client
        .create_multiple_signed_urls("list_files", vec!["1.txt", "2.txt", "3.txt"], 12431234)
        .await
        .unwrap();

    assert!(urls.len() >= 3)
}

#[tokio::test]
async fn test_get_public_url() {
    let client = create_test_client().await;

    client
        .get_public_url("list_files", "1.txt", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_move_file() {
    let client = create_test_client().await;

    // Move file
    let moved = client
        .move_file("list_files", None, "1.txt", "moved.txt")
        .await
        .unwrap();

    assert_eq!(moved, "Successfully moved");

    // Put it back
    let moved_back = client
        .move_file("list_files", None, "moved.txt", "1.txt")
        .await
        .unwrap();

    assert_eq!(moved_back, "Successfully moved");
}
