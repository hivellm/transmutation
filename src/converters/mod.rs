//! Document converters for various formats

pub mod traits;

// Converters (to be implemented)
#[cfg(feature = "pdf")]
pub mod pdf;

#[cfg(feature = "office")]
pub mod docx;

#[cfg(feature = "office")]
pub mod xlsx;

#[cfg(feature = "office")]
pub mod pptx;

#[cfg(feature = "web")]
pub mod html;

#[cfg(feature = "web")]
pub mod xml;

#[cfg(feature = "image-ocr")]
pub mod image;

#[cfg(feature = "archives")]
pub mod archive;

pub use traits::{ConverterMetadata, DocumentConverter};

