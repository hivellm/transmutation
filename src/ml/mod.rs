/// Machine Learning module for document layout analysis
/// 
/// This module provides ONNX-based ML models for:
/// - Layout detection (LayoutModel)
/// - Table structure recognition (TableStructureModel)
/// 
/// All models are optional and only compiled when `docling-ffi` feature is enabled.

#[cfg(feature = "docling-ffi")]
pub mod preprocessing;

#[cfg(feature = "docling-ffi")]
pub mod layout_model;

#[cfg(feature = "docling-ffi")]
pub mod table_structure_model;

#[cfg(feature = "docling-ffi")]
pub mod model_manager;

#[cfg(feature = "docling-ffi")]
pub use layout_model::LayoutModel;

#[cfg(feature = "docling-ffi")]
pub use table_structure_model::TableStructureModel;

#[cfg(feature = "docling-ffi")]
pub use model_manager::ModelManager;

use crate::error::Result;

/// Trait for ML models that process document pages
#[cfg(feature = "docling-ffi")]
pub trait DocumentModel {
    /// Model input type
    type Input;
    /// Model output type
    type Output;
    
    /// Run inference on input
    fn predict(&self, input: &Self::Input) -> Result<Self::Output>;
    
    /// Get model name for logging
    fn name(&self) -> &str;
}

