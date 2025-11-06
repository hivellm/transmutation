use std::path::Path;

use ndarray::Array4;
#[cfg(feature = "docling-ffi")]
use ort::{
    session::{Session, builder::SessionBuilder},
    value::Tensor,
};

#![allow(clippy::unused_self, clippy::unnecessary_wraps)]

/// Table structure recognition model using ONNX
///
/// Recognizes internal structure of tables (rows, columns, cells)
/// Based on docling's TableFormer model
use crate::error::{Result, TransmutationError};
use crate::ml::{DocumentModel, preprocessing};

/// Table cell in predicted structure
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct TableCell {
    pub row: usize,
    pub col: usize,
    pub row_span: usize,
    pub col_span: usize,
    pub bbox: (f32, f32, f32, f32), // (x0, y0, x1, y1)
    pub is_header: bool,
}

/// Table structure prediction output
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct TableStructure {
    pub cells: Vec<TableCell>,
    pub num_rows: usize,
    pub num_cols: usize,
}

/// Input for table structure model
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct TableInput {
    pub image: image::DynamicImage,
    pub table_bbox: (f32, f32, f32, f32),
}

/// ONNX-based table structure model
#[derive(Debug)]
pub struct TableStructureModel {
    #[cfg(feature = "docling-ffi")]
    session: Session,
    model_path: std::path::PathBuf,
    scale: f32, // Upscaling factor (typically 2.0 for 144 DPI)
}

impl TableStructureModel {
    /// Load table structure model from ONNX file
    ///
    /// `scale`: upscaling factor (2.0 = 144 DPI)
    pub fn new<P: AsRef<Path>>(model_path: P, scale: f32) -> Result<Self> {
        let model_path = model_path.as_ref().to_path_buf();

        #[cfg(feature = "docling-ffi")]
        {
            let session = SessionBuilder::new()?
                .with_intra_threads(4)?
                .commit_from_file(&model_path)
                .map_err(|e| TransmutationError::EngineError {
                    engine: "table-structure-model".to_string(),
                    message: format!("Failed to load ONNX model: {e}"),
                    source: None,
                })?;

            Ok(Self {
                session,
                model_path,
                scale,
            })
        }

        #[cfg(not(feature = "docling-ffi"))]
        {
            Err(TransmutationError::EngineError(
                "table-structure-model".to_string(),
                "docling-ffi feature not enabled".to_string(),
            ))
        }
    }

    /// Run inference on table region
    #[cfg(feature = "docling-ffi")]
    fn run_inference(&mut self, input: &Array4<f32>) -> Result<TableStructure> {
        // Convert ndarray to ONNX tensor (ort v2 API)
        // Extract shape and data as Vec for compatibility with OwnedTensorArrayData
        let shape = input.shape().to_vec();
        let data = input.iter().copied().collect::<Vec<f32>>();
        let input_tensor = Tensor::from_array((shape, data))?;

        // Run inference (ort v2 requires mutable session)
        // Extract outputs in a separate scope to end mutable borrow
        let (row_data, row_shape, col_data, col_shape, cell_data, cell_shape) = {
            let outputs = self.session.run(ort::inputs![input_tensor])?;
            let (rs, rd) = outputs[0].try_extract_tensor::<f32>()?;
            let (cs, cd) = outputs[1].try_extract_tensor::<f32>()?;
            let (cells, celld) = outputs[2].try_extract_tensor::<f32>()?;
            (
                rd.to_vec(),
                rs.to_vec(),
                cd.to_vec(),
                cs.to_vec(),
                celld.to_vec(),
                cells.to_vec(),
            )
        };

        // Now process with immutable borrow
        self.post_process_from_data(
            &row_shape,
            &row_data,
            &col_shape,
            &col_data,
            &cell_shape,
            &cell_data,
        )
    }

    #[cfg(feature = "docling-ffi")]
    fn post_process_from_data(
        &self,
        row_shape: &[i64],
        row_data: &[f32],
        col_shape: &[i64],
        col_data: &[f32],
        cell_shape: &[i64],
        cell_data: &[f32],
    ) -> Result<TableStructure> {
        // Reconstruct ndarrays from shape and data
        use ndarray::{ArrayD, IxDyn};
        let row_logits_array = ArrayD::from_shape_vec(
            IxDyn(
                row_shape
                    .iter()
                    .map(|&d| d as usize)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            row_data.to_vec(),
        )
        .map_err(|e| crate::TransmutationError::EngineError {
            engine: "table-structure-model".to_string(),
            message: format!("Failed to reshape row tensor: {e}"),
            source: None,
        })?;

        let col_logits_array = ArrayD::from_shape_vec(
            IxDyn(
                col_shape
                    .iter()
                    .map(|&d| d as usize)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            col_data.to_vec(),
        )
        .map_err(|e| crate::TransmutationError::EngineError {
            engine: "table-structure-model".to_string(),
            message: format!("Failed to reshape col tensor: {e}"),
            source: None,
        })?;

        let cell_logits_array = ArrayD::from_shape_vec(
            IxDyn(
                cell_shape
                    .iter()
                    .map(|&d| d as usize)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            cell_data.to_vec(),
        )
        .map_err(|e| crate::TransmutationError::EngineError {
            engine: "table-structure-model".to_string(),
            message: format!("Failed to reshape cell tensor: {e}"),
            source: None,
        })?;

        // Parse row and column structure
        let rows = self.parse_structure_logits(&row_logits_array.view())?;
        let cols = self.parse_structure_logits(&col_logits_array.view())?;

        // Build cell grid
        let cells = self.build_cell_grid(&rows, &cols, &cell_logits_array.view())?;

        Ok(TableStructure {
            cells,
            num_rows: rows.len(),
            num_cols: cols.len(),
        })
    }

    /// Parse structure logits to extract row/column positions
    #[cfg(feature = "docling-ffi")]
    fn parse_structure_logits(
        &self,
        logits: &ndarray::ArrayView<f32, ndarray::Dim<ndarray::IxDynImpl>>,
    ) -> Result<Vec<f32>> {
        // logits shape: [batch, sequence_length]
        let shape = logits.shape();
        if shape.len() < 2 {
            return Ok(Vec::new());
        }

        let seq_length = shape[1];
        let threshold = 0.5;

        let mut positions = Vec::new();

        // Extract positions where confidence > threshold
        for i in 0..seq_length {
            let value = logits[[0, i]];
            if value > threshold {
                positions.push(i as f32);
            }
        }

        // If no clear structure detected, use heuristics
        if positions.is_empty() {
            // Default: assume uniform distribution
            for i in 0..seq_length.min(10) {
                positions.push(i as f32);
            }
        }

        Ok(positions)
    }

    /// Build cell grid from row and column structure
    #[cfg(feature = "docling-ffi")]
    fn build_cell_grid(
        &self,
        rows: &[f32],
        cols: &[f32],
        cell_logits: &ndarray::ArrayView<f32, ndarray::Dim<ndarray::IxDynImpl>>,
    ) -> Result<Vec<TableCell>> {
        let mut cells = Vec::new();

        let num_rows = rows.len();
        let num_cols = cols.len();

        if num_rows == 0 || num_cols == 0 {
            return Ok(cells);
        }

        // Create cells for each grid position
        for row in 0..num_rows {
            for col in 0..num_cols {
                // Calculate cell bbox based on row/column positions
                let y0 = if row > 0 { rows[row - 1] } else { 0.0 };
                let y1 = rows[row];
                let x0 = if col > 0 { cols[col - 1] } else { 0.0 };
                let x1 = cols[col];

                // Detect spans (simplified - could be enhanced with cell_logits)
                let (row_span, col_span) =
                    self.detect_cell_spans(row, col, num_rows, num_cols, cell_logits);

                // Detect if this is a header cell (first row typically)
                let is_header = row == 0;

                cells.push(TableCell {
                    row,
                    col,
                    row_span,
                    col_span,
                    bbox: (x0, y0, x1, y1),
                    is_header,
                });
            }
        }

        Ok(cells)
    }

    /// Detect cell spans using cell logits
    #[cfg(feature = "docling-ffi")]
    fn detect_cell_spans(
        &self,
        _row: usize,
        _col: usize,
        _num_rows: usize,
        _num_cols: usize,
        _cell_logits: &ndarray::ArrayView<f32, ndarray::Dim<ndarray::IxDynImpl>>,
    ) -> (usize, usize) {
        // Simplified span detection
        // In production, analyze cell_logits to detect merged cells

        // For now, assume no spans (each cell is 1x1)
        // This can be enhanced by analyzing the cell_logits tensor
        // to detect cells that span multiple rows/columns

        (1, 1) // (row_span, col_span)
    }
}

#[cfg(feature = "docling-ffi")]
impl DocumentModel for TableStructureModel {
    type Input = TableInput;
    type Output = TableStructure;

    fn predict(&mut self, input: &Self::Input) -> Result<Self::Output> {
        // Extract table region from image
        let (x0, y0, x1, y1) = input.table_bbox;
        let table_img =
            input
                .image
                .crop_imm(x0 as u32, y0 as u32, (x1 - x0) as u32, (y1 - y0) as u32);

        // Preprocess with upscaling
        let tensor = preprocessing::preprocess_for_table(&table_img, self.scale)?;

        // Run inference
        let structure = self.run_inference(&tensor)?;

        Ok(structure)
    }

    fn name(&self) -> &str {
        "TableStructureModel"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires actual ONNX model file
    fn test_load_model() {
        let _result = TableStructureModel::new("models/tableformer_fast.onnx", 2.0);
        // Will fail if model doesn't exist, which is expected
    }
}
