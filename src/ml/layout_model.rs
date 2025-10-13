/// Layout detection model using ONNX
/// 
/// Detects document regions: text, tables, figures, headers, etc.
/// Based on docling's LayoutModel (docling_ibm_models)
use crate::error::{Result, TransmutationError};
use crate::ml::{DocumentModel, preprocessing};
use ndarray::Array4;
use std::path::Path;

#[cfg(feature = "docling-ffi")]
use ort::{Session, SessionBuilder, Value};

/// Document layout regions detected by the model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutLabel {
    Text,
    Title,
    SectionHeader,
    ListItem,
    Caption,
    Footnote,
    PageHeader,
    PageFooter,
    Table,
    Figure,
    Formula,
    Code,
}

/// Bounding box for detected region
#[derive(Debug, Clone)]
pub struct DetectedRegion {
    pub label: LayoutLabel,
    pub bbox: (f32, f32, f32, f32), // (x0, y0, x1, y1)
    pub confidence: f32,
}

/// Layout model output
#[derive(Debug, Clone)]
pub struct LayoutPrediction {
    pub regions: Vec<DetectedRegion>,
    pub page_width: u32,
    pub page_height: u32,
}

/// ONNX-based layout detection model
pub struct LayoutModel {
    #[cfg(feature = "docling-ffi")]
    session: Session,
    model_path: std::path::PathBuf,
}

impl LayoutModel {
    /// Load layout model from ONNX file
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let model_path = model_path.as_ref().to_path_buf();
        
        #[cfg(feature = "docling-ffi")]
        {
            let session = SessionBuilder::new()?
                .with_intra_threads(4)?
                .commit_from_file(&model_path)
                .map_err(|e| TransmutationError::EngineError(
                    "layout-model".to_string(),
                    format!("Failed to load ONNX model: {}", e)
                ))?;
            
            Ok(Self { session, model_path })
        }
        
        #[cfg(not(feature = "docling-ffi"))]
        {
            Err(TransmutationError::EngineError(
                "layout-model".to_string(),
                "docling-ffi feature not enabled".to_string()
            ))
        }
    }
    
    /// Run inference on preprocessed image
    #[cfg(feature = "docling-ffi")]
    fn run_inference(&self, input: &Array4<f32>) -> Result<Vec<DetectedRegion>> {
        // Convert ndarray to ONNX tensor
        let input_tensor = Value::from_array(input.clone())?;
        
        // Run inference
        let outputs = self.session.run(vec![input_tensor])?;
        
        // Extract predictions (placeholder - actual post-processing needed)
        // TODO: Implement mask-to-bbox conversion
        let regions = self.post_process_output(&outputs)?;
        
        Ok(regions)
    }
    
    #[cfg(feature = "docling-ffi")]
    fn post_process_output(&self, _outputs: &[Value]) -> Result<Vec<DetectedRegion>> {
        // TODO: Implement actual post-processing
        // 1. Extract segmentation masks
        // 2. Convert masks to bounding boxes
        // 3. Apply NMS (non-maximum suppression)
        // 4. Map class IDs to LayoutLabel
        
        // Placeholder
        Ok(Vec::new())
    }
}

#[cfg(feature = "docling-ffi")]
impl DocumentModel for LayoutModel {
    type Input = image::DynamicImage;
    type Output = LayoutPrediction;
    
    fn predict(&self, input: &Self::Input) -> Result<Self::Output> {
        // Preprocess image
        let tensor = preprocessing::preprocess_for_layout(input)?;
        
        // Run inference
        let regions = self.run_inference(&tensor)?;
        
        let (width, height) = input.dimensions();
        
        Ok(LayoutPrediction {
            regions,
            page_width: width,
            page_height: height,
        })
    }
    
    fn name(&self) -> &str {
        "LayoutModel"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Requires actual ONNX model file
    fn test_load_model() {
        let result = LayoutModel::new("models/layout_model.onnx");
        // Will fail if model doesn't exist, which is expected
    }
}

