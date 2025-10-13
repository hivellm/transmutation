/// Layout detection model using ONNX
/// 
/// Detects document regions: text, tables, figures, headers, etc.
/// Based on docling's LayoutModel (docling_ibm_models)
use crate::error::{Result, TransmutationError};
use crate::ml::{DocumentModel, preprocessing};
use ndarray::Array4;
use std::path::Path;
use image::GenericImageView;

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
    fn post_process_output(&self, outputs: &[Value]) -> Result<Vec<DetectedRegion>> {
        // Extract segmentation masks from ONNX output
        // Output format: [batch, num_classes, height, width]
        if outputs.is_empty() {
            return Ok(Vec::new());
        }
        
        // Get the output tensor
        let output_tensor = &outputs[0];
        
        // Extract tensor data as ndarray
        // The output is a segmentation mask with shape [1, num_classes, H, W]
        let masks = output_tensor.try_extract_tensor::<f32>()?;
        let shape = masks.shape();
        
        if shape.len() != 4 {
            return Err(crate::TransmutationError::EngineError(
                "layout-model".to_string(),
                format!("Expected 4D output tensor, got {}D", shape.len())
            ));
        }
        
        let num_classes = shape[1];
        let height = shape[2];
        let width = shape[3];
        
        // Process each class mask
        let mut all_regions = Vec::new();
        
        for class_id in 0..num_classes {
            // Extract mask for this class
            let class_mask = masks.slice(ndarray::s![0, class_id, .., ..]);
            
            // Convert mask to regions using connected components
            let regions = self.mask_to_regions(&class_mask, class_id, width, height)?;
            all_regions.extend(regions);
        }
        
        // Apply Non-Maximum Suppression to remove overlapping detections
        let filtered_regions = self.apply_nms(all_regions, 0.5)?;
        
        Ok(filtered_regions)
    }
    
    /// Convert binary mask to bounding box regions using connected components
    #[cfg(feature = "docling-ffi")]
    fn mask_to_regions(
        &self,
        mask: &ndarray::ArrayView2<f32>,
        class_id: usize,
        width: usize,
        height: usize,
    ) -> Result<Vec<DetectedRegion>> {
        let threshold = 0.5; // Confidence threshold
        let mut regions = Vec::new();
        
        // Simple threshold-based approach
        // For production, use connected components algorithm
        let mut visited = vec![vec![false; width]; height];
        
        for y in 0..height {
            for x in 0..width {
                if mask[[y, x]] > threshold && !visited[y][x] {
                    // Start a new region
                    let bbox = self.flood_fill_bbox(mask, &mut visited, x, y, width, height, threshold);
                    
                    if let Some((x0, y0, x1, y1)) = bbox {
                        // Map class_id to LayoutLabel
                        if let Some(label) = self.class_id_to_label(class_id) {
                            // Calculate confidence (average of mask values in bbox)
                            let confidence = self.calculate_region_confidence(mask, x0, y0, x1, y1);
                            
                            regions.push(DetectedRegion {
                                label,
                                bbox: (x0 as f32, y0 as f32, x1 as f32, y1 as f32),
                                confidence,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(regions)
    }
    
    /// Flood fill to find connected component bounding box
    #[cfg(feature = "docling-ffi")]
    fn flood_fill_bbox(
        &self,
        mask: &ndarray::ArrayView2<f32>,
        visited: &mut Vec<Vec<bool>>,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        threshold: f32,
    ) -> Option<(usize, usize, usize, usize)> {
        let mut stack = vec![(start_x, start_y)];
        let mut min_x = start_x;
        let mut min_y = start_y;
        let mut max_x = start_x;
        let mut max_y = start_y;
        
        while let Some((x, y)) = stack.pop() {
            if x >= width || y >= height || visited[y][x] || mask[[y, x]] <= threshold {
                continue;
            }
            
            visited[y][x] = true;
            
            // Update bounding box
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            
            // Add neighbors (4-connectivity)
            if x > 0 { stack.push((x - 1, y)); }
            if x + 1 < width { stack.push((x + 1, y)); }
            if y > 0 { stack.push((x, y - 1)); }
            if y + 1 < height { stack.push((x, y + 1)); }
        }
        
        // Filter out very small regions
        if (max_x - min_x) < 5 || (max_y - min_y) < 5 {
            return None;
        }
        
        Some((min_x, min_y, max_x, max_y))
    }
    
    /// Calculate average confidence in region
    #[cfg(feature = "docling-ffi")]
    fn calculate_region_confidence(
        &self,
        mask: &ndarray::ArrayView2<f32>,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
    ) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;
        
        for y in y0..=y1 {
            for x in x0..=x1 {
                if y < mask.shape()[0] && x < mask.shape()[1] {
                    sum += mask[[y, x]];
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            sum / count as f32
        } else {
            0.0
        }
    }
    
    /// Apply Non-Maximum Suppression to filter overlapping regions
    #[cfg(feature = "docling-ffi")]
    fn apply_nms(&self, mut regions: Vec<DetectedRegion>, iou_threshold: f32) -> Result<Vec<DetectedRegion>> {
        // Sort by confidence (descending)
        regions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        let mut keep = Vec::new();
        let mut suppressed = vec![false; regions.len()];
        
        for i in 0..regions.len() {
            if suppressed[i] {
                continue;
            }
            
            keep.push(regions[i].clone());
            
            // Suppress overlapping regions
            for j in (i + 1)..regions.len() {
                if suppressed[j] {
                    continue;
                }
                
                let iou = self.calculate_iou(&regions[i].bbox, &regions[j].bbox);
                if iou > iou_threshold {
                    suppressed[j] = true;
                }
            }
        }
        
        Ok(keep)
    }
    
    /// Calculate Intersection over Union (IoU)
    #[cfg(feature = "docling-ffi")]
    fn calculate_iou(&self, bbox1: &(f32, f32, f32, f32), bbox2: &(f32, f32, f32, f32)) -> f32 {
        let (x1_min, y1_min, x1_max, y1_max) = bbox1;
        let (x2_min, y2_min, x2_max, y2_max) = bbox2;
        
        // Calculate intersection
        let inter_x_min = x1_min.max(*x2_min);
        let inter_y_min = y1_min.max(*y2_min);
        let inter_x_max = x1_max.min(*x2_max);
        let inter_y_max = y1_max.min(*y2_max);
        
        if inter_x_max <= inter_x_min || inter_y_max <= inter_y_min {
            return 0.0;
        }
        
        let inter_area = (inter_x_max - inter_x_min) * (inter_y_max - inter_y_min);
        
        // Calculate union
        let area1 = (x1_max - x1_min) * (y1_max - y1_min);
        let area2 = (x2_max - x2_min) * (y2_max - y2_min);
        let union_area = area1 + area2 - inter_area;
        
        if union_area > 0.0 {
            inter_area / union_area
        } else {
            0.0
        }
    }
    
    /// Map class ID to LayoutLabel
    /// Based on docling's class definitions
    #[cfg(feature = "docling-ffi")]
    fn class_id_to_label(&self, class_id: usize) -> Option<LayoutLabel> {
        match class_id {
            0 => Some(LayoutLabel::Text),
            1 => Some(LayoutLabel::Title),
            2 => Some(LayoutLabel::SectionHeader),
            3 => Some(LayoutLabel::ListItem),
            4 => Some(LayoutLabel::Caption),
            5 => Some(LayoutLabel::Footnote),
            6 => Some(LayoutLabel::PageHeader),
            7 => Some(LayoutLabel::PageFooter),
            8 => Some(LayoutLabel::Table),
            9 => Some(LayoutLabel::Figure),
            10 => Some(LayoutLabel::Formula),
            11 => Some(LayoutLabel::Code),
            _ => None, // Unknown class
        }
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

