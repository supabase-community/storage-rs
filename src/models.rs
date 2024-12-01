use std::{fmt, time::Duration};

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Supabase Storage Client
pub struct StorageClient {
    pub client: Client,
    /// REST endpoint for querying and managing your database
    /// Example: https://<project id>.supabase.co
    pub project_url: String,
    /// WARN: The `service role` key has the ability to bypass Row Level Security. Never share it publicly.
    pub api_key: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CreateBucket<'a> {
    /// The ID of the bucket used for making updates or deletion
    pub id: Option<&'a str>,
    /// The visible name of the bucket in your dashboard
    pub name: &'a str,
    /// The visibility of the bucket. Public buckets don't require an authorization token to download objects, but still require a valid token for all other operations.
    pub public: bool,
    /// the allowed mime types that this bucket can accept during upload. The default value is null, which allows files with all mime types to be uploaded.
    // pub allowed_mime_types: Option<Vec<&'a str>>,
    pub allowed_mime_types: Option<Vec<String>>,
    /// The max file size in bytes that can be uploaded to this bucket. The global file size limit takes precedence over this value. No maximum size is set by default.
    pub file_size_limit: Option<u64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct UpdateBucket<'a> {
    /// The ID of the bucket used for making updates or deletion
    pub id: &'a str,
    /// The visibility of the bucket. Public buckets don't require an authorization token to download objects, but still require a valid token for all other operations.
    pub public: bool,
    /// the allowed mime types that this bucket can accept during upload. The default value is null, which allows files with all mime types to be uploaded.
    // pub allowed_mime_types: Option<Vec<&'a str>>,
    pub allowed_mime_types: Option<Vec<String>>,
    /// The max file size in bytes that can be uploaded to this bucket. The global file size limit takes precedence over this value. No maximum size is set by default.
    pub file_size_limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileObject {
    pub name: String,
    pub id: String,
    pub updated_at: String,
    pub created_at: String,
    pub last_accessed_at: String,
    pub metadata: Metadata,
    pub bucket_id: Option<String>,
    pub owner: Option<String>,
    pub buckets: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "eTag")]
    pub etag: String,
    pub size: i32,
    pub mimetype: String,
    #[serde(rename = "cacheControl")]
    pub cache_control: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "contentLength")]
    pub content_length: i32,
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileSearchOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of files you want to be returned
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The starting position
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortBy")]
    /// The column to sort by. Can be any column inside a FileObject
    pub sort_by: Option<SortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///  Performs a full-text search across object names. For example,
    ///   with search "photo", it will match:
    ///   - family-photo-2024.jpg
    ///   - uploads/photo1.png
    ///   - photos/vacation/beach.jpg
    pub search: Option<&'a str>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortBy {
    pub column: Column,
    pub order: Order,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    #[default]
    Asc,
    Desc,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Column {
    #[default]
    Name,
    ID,
    UpdatedAt,
    CreatedAt,
    LastAccessedAt,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CreateSignedUrlPayload {
    #[serde(rename = "expiresIn")]
    pub(crate) expires_in: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CreateMultipleSignedUrlsPayload<'a> {
    #[serde(rename = "expiresIn")]
    pub(crate) expires_in: u64,
    #[serde(borrow)]
    pub(crate) paths: Vec<&'a str>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignedUrlResponse {
    #[serde(rename = "signedURL")]
    pub signed_url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignedUploadUrlResponse {
    pub url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CopyFilePayload<'a> {
    #[serde(rename = "bucketId")]
    pub(crate) bucket_id: &'a str,
    #[serde(rename = "sourceKey")]
    pub(crate) source_key: &'a str,
    #[serde(rename = "destinationBucket")]
    pub(crate) destination_bucket: &'a str,
    #[serde(rename = "destinationKey")]
    pub(crate) destination_key: &'a str,
    #[serde(rename = "copyMetadata")]
    pub(crate) copy_metadata: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CopyFileResponse {
    #[serde(rename = "Key")]
    pub key: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadToSignedUrlResponse {
    #[serde(rename = "Key")]
    pub key: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ListFilesPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of files you want to be returned
    pub(crate) limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The starting position
    pub(crate) offset: Option<u32>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The column to sort by. Can be any column inside a FileObject.
    pub(crate) sort_by: Option<SortBy>,
    /// Filters objects that start with this exact string. For example,
    ///   with prefix "uploads/2024/", it will match:
    ///   - uploads/2024/image1.jpg
    ///   - uploads/2024/january/photo.png
    ///   but not:
    ///   - uploads/2023/image1.jpg
    ///   - documents/2024/file.pdf
    pub(crate) prefix: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///  Performs a full-text search across object names. For example,
    ///   with search "photo", it will match:
    ///   - family-photo-2024.jpg
    ///   - uploads/photo1.png
    ///   - photos/vacation/beach.jpg
    pub(crate) search: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct DownloadOptions<'a> {
    pub transform: Option<TransformOptions<'a>>,
    pub download: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformOptions<'a> {
    /// The width of the image in pixels
    pub width: Option<u64>,
    /// The height of the image in pixels
    pub height: Option<u64>,
    /// The resize mode can be cover, contain or fill. Defaults to cover.
    /// Cover resizes the image to maintain it's aspect ratio while filling the entire width and height.
    /// Contain resizes the image to maintain it's aspect ratio while fitting the entire image within the width and height.
    /// Fill resizes the image to fill the entire width and height. If the object's aspect ratio does not match the width and height, the image will be stretched to fit.
    pub resize: Option<&'a str>,
    /// Specify the format of the image requested.
    ///
    /// When using 'origin' we force the format to be the same as the original image.
    /// When this option is not passed in, images are optimized to modern image formats like Webp.
    pub format: Option<&'a str>,
    /// Sets the quality of the returned image
    ///
    /// A number from 20 to 100, with 100 being the highest quality. Defaults to 80
    pub quality: Option<u8>,
}

/// Configuration options for file uploads to Supabase Storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Upload<'a> {
    /// The file path, including the file name (format: folder/subfolder/filename.png)
    /// The bucket must already exist before attempting to upload.
    pub path: &'a str,
    /// The body of the file to be stored in the bucket
    pub file_body: Vec<u8>,
    /// Optional file configuration settings
    pub file_options: Option<FileOptions<'a>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileOptions<'a> {
    /// The number of seconds the asset is cached in the browser and Supabase CDN
    /// Sets the Cache-Control: max-age=<seconds> header
    /// Defaults to 3600 seconds
    pub cache_control: Option<Duration>,
    /// The Content-Type header value
    /// Required if using a fileBody that is neither Blob, File, nor FormData
    /// Defaults to "text/plain;charset=UTF-8"
    pub content_type: Option<&'a str>,
    /// Enables or disables duplex streaming for reading and writing data in the same stream
    pub duplex: Option<&'a str>,
    /// When true, the file is overwritten if it exists
    /// When false, an error is thrown if the object already exists
    /// Defaults to false
    pub upsert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectResponse {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Key")]
    pub key: String,
}

pub type Buckets = Vec<Bucket>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBucketResponse {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BucketResponse {
    pub(crate) message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MimeType<'a> {
    Custom(&'a str),
    AAC,
    AbiWord,
    APNG,
    Archive,
    AVIF,
    AVI,
    AmazonKindle,
    BinaryData,
    BMP,
    BZip,
    BZip2,
    CDAudio,
    CShellScript,
    CSS,
    CSV,
    DOC,
    DOCX,
    EOT,
    EPUB,
    GZip,
    GIF,
    HTML,
    Icon,
    ICalendar,
    JAR,
    JPEG,
    JavaScript,
    JSON,
    JSONLD,
    MIDI,
    JavaScriptModule,
    MP3,
    MP4,
    MPEG,
    AppleInstaller,
    ODP,
    ODS,
    ODT,
    OggAudio,
    OggVideo,
    Ogg,
    OpusAudio,
    OTF,
    PNG,
    PDF,
    PHP,
    PPT,
    PPTX,
    RAR,
    RTF,
    ShellScript,
    SVG,
    TAR,
    TIFF,
    MPEGTransportStream,
    TTF,
    PlainText,
    Visio,
    WAV,
    WEBMAudio,
    WEBMVideo,
    WEBP,
    WOFF,
    WOFF2,
    XHTML,
    XLS,
    XLSX,
    XML,
    XUL,
    ZIP,
    ThreeGPP,
    ThreeGPP2,
    SevenZip,
}

impl MimeType<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            MimeType::AAC => "audio/aac",
            MimeType::AbiWord => "application/x-abiword",
            MimeType::APNG => "image/apng",
            MimeType::Archive => "application/x-freearc",
            MimeType::AVIF => "image/avif",
            MimeType::AVI => "video/x-msvideo",
            MimeType::AmazonKindle => "application/vnd.amazon.ebook",
            MimeType::BinaryData => "application/octet-stream",
            MimeType::BMP => "image/bmp",
            MimeType::BZip => "application/x-bzip",
            MimeType::BZip2 => "application/x-bzip2",
            MimeType::CDAudio => "application/x-cdf",
            MimeType::CShellScript => "application/x-csh",
            MimeType::CSS => "text/css",
            MimeType::CSV => "text/csv",
            MimeType::DOC => "application/msword",
            MimeType::DOCX => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            MimeType::EOT => "application/vnd.ms-fontobject",
            MimeType::EPUB => "application/epub+zip",
            MimeType::GZip => "application/gzip",
            MimeType::GIF => "image/gif",
            MimeType::HTML => "text/html",
            MimeType::Icon => "image/vnd.microsoft.icon",
            MimeType::ICalendar => "text/calendar",
            MimeType::JAR => "application/java-archive",
            MimeType::JPEG => "image/jpeg",
            MimeType::JavaScript => "text/javascript",
            MimeType::JSON => "application/json",
            MimeType::JSONLD => "application/ld+json",
            MimeType::MIDI => "audio/midi",
            MimeType::JavaScriptModule => "text/javascript",
            MimeType::MP3 => "audio/mpeg",
            MimeType::MP4 => "video/mp4",
            MimeType::MPEG => "video/mpeg",
            MimeType::AppleInstaller => "application/vnd.apple.installer+xml",
            MimeType::ODP => "application/vnd.oasis.opendocument.presentation",
            MimeType::ODS => "application/vnd.oasis.opendocument.spreadsheet",
            MimeType::ODT => "application/vnd.oasis.opendocument.text",
            MimeType::OggAudio => "audio/ogg",
            MimeType::OggVideo => "video/ogg",
            MimeType::Ogg => "application/ogg",
            MimeType::OpusAudio => "audio/ogg",
            MimeType::OTF => "font/otf",
            MimeType::PNG => "image/png",
            MimeType::PDF => "application/pdf",
            MimeType::PHP => "application/x-httpd-php",
            MimeType::PPT => "application/vnd.ms-powerpoint",
            MimeType::PPTX => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            MimeType::RAR => "application/vnd.rar",
            MimeType::RTF => "application/rtf",
            MimeType::ShellScript => "application/x-sh",
            MimeType::SVG => "image/svg+xml",
            MimeType::TAR => "application/x-tar",
            MimeType::TIFF => "image/tiff",
            MimeType::MPEGTransportStream => "video/mp2t",
            MimeType::TTF => "font/ttf",
            MimeType::PlainText => "text/plain",
            MimeType::Visio => "application/vnd.visio",
            MimeType::WAV => "audio/wav",
            MimeType::WEBMAudio => "audio/webm",
            MimeType::WEBMVideo => "video/webm",
            MimeType::WEBP => "image/webp",
            MimeType::WOFF => "font/woff",
            MimeType::WOFF2 => "font/woff2",
            MimeType::XHTML => "application/xhtml+xml",
            MimeType::XLS => "application/vnd.ms-excel",
            MimeType::XLSX => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            MimeType::XML => "application/xml",
            MimeType::XUL => "application/vnd.mozilla.xul+xml",
            MimeType::ZIP => "application/zip",
            MimeType::ThreeGPP => "video/3gpp",
            MimeType::ThreeGPP2 => "video/3gpp2",
            MimeType::SevenZip => "application/x-7z-compressed",
            MimeType::Custom(mime) => &mime,
        }
    }
}

impl fmt::Display for MimeType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<MimeType<'_>> for String {
    fn from(mime: MimeType) -> Self {
        mime.to_string()
    }
}

pub const HEADER_API_KEY: &str = "apikey";
pub const STORAGE_V1: &str = "/storage/v1";
