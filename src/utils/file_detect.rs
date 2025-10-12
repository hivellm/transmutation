//! File type detection utilities

use crate::types::FileFormat;
use crate::{Result, TransmutationError};
use std::path::Path;

/// Detect file format from path
pub async fn detect_format<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
    let path = path.as_ref();

    // First, try magic byte detection
    if let Ok(format) = detect_by_magic_bytes(path).await {
        return Ok(format);
    }

    // Fallback to extension-based detection
    detect_by_extension(path)
}

/// Detect format by reading magic bytes
async fn detect_by_magic_bytes(path: &Path) -> Result<FileFormat> {
    use file_format::FileFormat as FFFormat;

    let data = tokio::fs::read(path).await?;
    let ff_format = FFFormat::from_bytes(&data);

    let format = match ff_format.media_type() {
        "application/pdf" => FileFormat::Pdf,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            FileFormat::Docx
        }
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
            FileFormat::Pptx
        }
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => FileFormat::Xlsx,
        "text/html" => FileFormat::Html,
        "text/xml" | "application/xml" => FileFormat::Xml,
        "text/plain" => FileFormat::Txt,
        "text/markdown" => FileFormat::Markdown,
        "text/csv" => FileFormat::Csv,
        "image/jpeg" => FileFormat::Jpeg,
        "image/png" => FileFormat::Png,
        "image/tiff" => FileFormat::Tiff,
        "image/bmp" => FileFormat::Bmp,
        "image/gif" => FileFormat::Gif,
        "image/webp" => FileFormat::Webp,
        "audio/mpeg" => FileFormat::Mp3,
        "audio/wav" | "audio/x-wav" => FileFormat::Wav,
        "audio/mp4" => FileFormat::M4a,
        "audio/flac" => FileFormat::Flac,
        "video/mp4" => FileFormat::Mp4,
        "video/x-msvideo" => FileFormat::Avi,
        "video/x-matroska" => FileFormat::Mkv,
        "video/quicktime" => FileFormat::Mov,
        "application/zip" => FileFormat::Zip,
        "application/x-tar" => FileFormat::Tar,
        "application/gzip" => FileFormat::TarGz,
        "application/x-7z-compressed" => FileFormat::SevenZ,
        _ => FileFormat::Unknown,
    };

    if format == FileFormat::Unknown {
        Err(TransmutationError::UnsupportedFormat(format!(
            "Unknown format: {}",
            ff_format.media_type()
        )))
    } else {
        Ok(format)
    }
}

/// Detect format by file extension
fn detect_by_extension(path: &Path) -> Result<FileFormat> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .ok_or_else(|| {
            TransmutationError::UnsupportedFormat("No file extension found".to_string())
        })?;

    let format = match extension.as_str() {
        "pdf" => FileFormat::Pdf,
        "docx" => FileFormat::Docx,
        "pptx" => FileFormat::Pptx,
        "xlsx" => FileFormat::Xlsx,
        "html" | "htm" => FileFormat::Html,
        "xml" => FileFormat::Xml,
        "txt" | "text" => FileFormat::Txt,
        "md" | "markdown" => FileFormat::Markdown,
        "rtf" => FileFormat::Rtf,
        "odt" => FileFormat::Odt,
        "csv" => FileFormat::Csv,
        "tsv" => FileFormat::Tsv,
        "jpg" | "jpeg" => FileFormat::Jpeg,
        "png" => FileFormat::Png,
        "tif" | "tiff" => FileFormat::Tiff,
        "bmp" => FileFormat::Bmp,
        "gif" => FileFormat::Gif,
        "webp" => FileFormat::Webp,
        "mp3" => FileFormat::Mp3,
        "wav" => FileFormat::Wav,
        "m4a" => FileFormat::M4a,
        "flac" => FileFormat::Flac,
        "mp4" => FileFormat::Mp4,
        "avi" => FileFormat::Avi,
        "mkv" => FileFormat::Mkv,
        "mov" => FileFormat::Mov,
        "zip" => FileFormat::Zip,
        "tar" => FileFormat::Tar,
        "gz" if path.file_name().and_then(|s| s.to_str()).map(|s| s.ends_with(".tar.gz")).unwrap_or(false) => {
            FileFormat::TarGz
        }
        "bz2" if path.file_name().and_then(|s| s.to_str()).map(|s| s.ends_with(".tar.bz2")).unwrap_or(false) => {
            FileFormat::TarBz2
        }
        "7z" => FileFormat::SevenZ,
        _ => FileFormat::Unknown,
    };

    if format == FileFormat::Unknown {
        Err(TransmutationError::UnsupportedFormat(format!(
            "Unsupported extension: .{}",
            extension
        )))
    } else {
        Ok(format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_detect_by_extension_pdf() {
        let path = PathBuf::from("test.pdf");
        let format = detect_by_extension(&path).unwrap();
        assert_eq!(format, FileFormat::Pdf);
    }

    #[test]
    fn test_detect_by_extension_docx() {
        let path = PathBuf::from("document.docx");
        let format = detect_by_extension(&path).unwrap();
        assert_eq!(format, FileFormat::Docx);
    }

    #[test]
    fn test_detect_by_extension_tar_gz() {
        let path = PathBuf::from("archive.tar.gz");
        let format = detect_by_extension(&path).unwrap();
        assert_eq!(format, FileFormat::TarGz);
    }

    #[test]
    fn test_detect_by_extension_unknown() {
        let path = PathBuf::from("file.xyz");
        let result = detect_by_extension(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_by_extension_no_extension() {
        let path = PathBuf::from("file");
        let result = detect_by_extension(&path);
        assert!(result.is_err());
    }
}

