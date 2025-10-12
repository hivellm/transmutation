//! Engine abstractions for document processing

#[cfg(feature = "pdf")]
pub mod pdf_parser;

#[cfg(feature = "pdf")]
pub mod table_detector;

#[cfg(feature = "pdf")]
pub mod layout_analyzer;

#[cfg(feature = "ml")]
pub mod docling_ml;

#[cfg(feature = "tesseract")]
pub mod tesseract;

#[cfg(feature = "ffmpeg")]
pub mod ffmpeg;

// Future engines
// pub mod audio_asr;
