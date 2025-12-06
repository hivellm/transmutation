//! Engine abstractions for document processing

#![allow(missing_docs)]

// Core engines (always enabled for PDF support)
pub mod layout_analyzer;
pub mod pdf_parser;
pub mod table_detector;

// Advanced FFI engines (optional)
#[cfg(feature = "docling-ffi")]
pub mod docling_parse_ffi;

#[cfg(feature = "docling-ffi")]
pub mod docling_json_parser;

#[cfg(feature = "docling-ffi")]
pub mod rule_based_layout;

#[cfg(feature = "docling-ffi")]
pub mod layout_postprocessor;

#[cfg(feature = "docling-ffi")]
pub use layout_postprocessor::LayoutPostprocessor;

// Note: Tesseract OCR is implemented in converters/image.rs
// Note: FFmpeg processing is out of scope for Transmutation
