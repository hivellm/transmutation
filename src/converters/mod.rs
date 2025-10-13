//! Document converters for various formats

pub mod traits;

// Core converters (always enabled)
pub mod pdf;
pub mod html;
pub mod xml;
pub mod archive;

// Office formats (optional)
#[cfg(feature = "office")]
pub mod docx;

#[cfg(feature = "office")]
pub mod xlsx;

#[cfg(feature = "office")]
pub mod pptx;

// Text formats (always enabled)
pub mod txt;
pub mod csv;
pub mod rtf;
pub mod odt;

// Advanced features (optional)
#[cfg(feature = "image-ocr")]
pub mod image;

pub use traits::{ConverterMetadata, DocumentConverter};

