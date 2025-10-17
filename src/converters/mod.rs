//! Document converters for various formats

pub mod traits;

// Core converters (always enabled)
pub mod archive;
pub mod html;
pub mod pdf;
pub mod xml;

// Office formats (optional)
#[cfg(feature = "office")]
pub mod docx;

#[cfg(feature = "office")]
pub mod xlsx;

#[cfg(feature = "office")]
pub mod pptx;

// Text formats (always enabled)
pub mod csv;
pub mod odt;
pub mod rtf;
pub mod txt;

// Advanced features (optional)
#[cfg(feature = "image-ocr")]
pub mod image;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "video")]
pub mod video;

pub use traits::{ConverterMetadata, DocumentConverter};
