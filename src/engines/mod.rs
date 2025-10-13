//! Engine abstractions for document processing

#[cfg(feature = "pdf")]
pub mod pdf_parser;

#[cfg(feature = "pdf")]
pub mod table_detector;

#[cfg(feature = "pdf")]
pub mod layout_analyzer;

#[cfg(all(feature = "pdf", feature = "docling-ffi"))]
pub mod docling_parse_ffi;

#[cfg(all(feature = "pdf", feature = "docling-ffi"))]
pub mod docling_json_parser;

#[cfg(feature = "docling-ffi")]
pub mod layout_postprocessor;

#[cfg(feature = "docling-ffi")]
pub use layout_postprocessor::LayoutPostprocessor;

#[cfg(feature = "tesseract")]
pub mod tesseract;

#[cfg(feature = "ffmpeg")]
pub mod ffmpeg;

// Future engines
// pub mod audio_asr;
