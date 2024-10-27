use std::fmt;

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
    pub id: Option<String>,
    /// The visible name of the bucket in your dashboard
    pub name: String,
    /// The visibility of the bucket. Public buckets don't require an authorization token to download objects, but still require a valid token for all other operations.
    pub public: bool,
    /// the allowed mime types that this bucket can accept during upload. The default value is null, which allows files with all mime types to be uploaded.
    pub allowed_mime_types: Option<Vec<String>>,
    /// The max file size in bytes that can be uploaded to this bucket. The global file size limit takes precedence over this value. No maximum size is set by default.
    pub file_size_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBucketResponse {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteBucketResponse {
    pub(crate) message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MimeType {
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

impl MimeType {
    pub fn as_str(&self) -> &'static str {
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
        }
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<MimeType> for String {
    fn from(mime: MimeType) -> Self {
        mime.to_string()
    }
}

pub const HEADER_API_KEY: &str = "apikey";
pub const STORAGE_V1: &str = "/storage/v1";
