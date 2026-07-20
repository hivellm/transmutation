# ML Model Activation Guide

**Status:** 🚧 Infrastructure Ready - Models Not Yet Active  
**Date:** October 13, 2025

This guide explains how to activate ML-based layout detection and table structure recognition when ONNX models are available.

---

## Current State

### ✅ Implemented (Ready)
- **Model Loading Infrastructure**: `src/ml/model_manager.rs`
  - Smart path resolution
  - Graceful fallback
  - Multiple search locations
  
- **Model Caching**: `src/ml/model_cache.rs`
  - Lazy loading
  - Session reuse
  - Memory management

- **Model Export Script**: `scripts/export_onnx_models.py`
  - Layout Model export
  - TableFormer export
  - Verification and optimization

- **ML Model Implementations**:
  - `src/ml/layout_model.rs` - Layout detection
  - `src/ml/table_structure_model.rs` - Table structure
  - `src/ml/preprocessing.rs` - Image preprocessing
  - `src/ml/cell_matching.rs` - Cell-to-table matching

### ⏸️ Not Yet Active
- ML model inference (commented out/fallback mode)
- Actual region detection using models
- Table structure prediction

---

## How to Activate ML Inference

### Step 1: Export ONNX Models

```bash
cd transmutation

# Install dependencies
pip install docling docling-ibm-models torch onnx onnxruntime

# Export models
python scripts/export_onnx_models.py
```

**Output:**
- `models/layout_model.onnx` (~100-200 MB)
- `models/table_structure_model.onnx` (~50-100 MB)

### Step 2: Verify Model Files

```bash
ls -lh models/
# Should show:
# layout_model.onnx
# table_structure_model.onnx
```

### Step 3: Enable ML Pipeline in Code

**File:** `src/converters/pdf.rs`

Currently, the code uses:
```rust
// Using parsed items directly (no ML clusters)
let items_to_use = doc.items;
```

To activate ML:
```rust
// [1] Load models from cache
use crate::ml::{get_layout_model, ModelManager};

let model_manager = ModelManager::new()?;
if let Some(layout_model_path) = model_manager.load_or_download(
    crate::ml::model_manager::LAYOUT_MODEL_NAME
) {
    // [2] Get or load cached model
    if let Some(layout_model) = get_layout_model(layout_model_path) {
        // [3] Run layout detection
        let mut model = layout_model.lock().unwrap();
        
        // Convert PDF page to image
        let page_image = render_pdf_page_to_image(pdf_path, page_num)?;
        
        // Run layout prediction
        let prediction = model.predict(&page_image)?;
        
        // [4] Process detected regions into clusters
        let clusters = prediction.regions.iter().map(|region| {
            Cluster {
                id: region.id,
                label: map_layout_label(region.label),
                bbox: region.bbox,
                cells: extract_cells_in_bbox(&doc.items, &region.bbox),
                confidence: region.confidence,
            }
        }).collect();
        
        // [5] Use ML-detected clusters
        let items_to_use = assembler.assemble(&clusters)?;
    }
}
```

### Step 4: Add PDF-to-Image Rendering

**Dependencies needed in `Cargo.toml`:**
```toml
pdfium-render = "0.8"  # Already included
image = "0.25"         # Already included
```

**Implementation:**
```rust
fn render_pdf_page_to_image(
    pdf_path: &Path, 
    page_num: usize
) -> Result<DynamicImage> {
    use pdfium_render::prelude::*;
    
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_file(pdf_path, None)?;
    let page = document.pages().get(page_num)?;
    
    // Render at 150 DPI (good balance)
    let bitmap = page.render_with_config(
        &PdfRenderConfig::new()
            .set_target_width(1024)
            .set_target_height(1024)
    )?;
    
    let image = bitmap.as_image();
    Ok(image)
}
```

### Step 5: Test ML Pipeline

```bash
# Build with models available
cargo build --release --features docling-ffi

# Run test
./target/release/examples/test_ffi

# Should see:
# ✅ Found layout_model.onnx at models/layout_model.onnx
# 🔄 Loading LayoutModel from models/layout_model.onnx
# ✅ LayoutModel loaded and cached
# [ML] Detected 15 regions (3 tables, 12 text blocks)
```

---

## Integration Points

### 1. Layout Detection Flow

```
PDF Page → Render to Image → Layout Model → Detected Regions → Clusters → DocItems
```

**Current:** Skip "Layout Model" step, use FFI JSON directly

**With ML:**
```rust
// In convert_with_docling_ffi()
for page_num in 0..num_pages {
    // Get raw cells from FFI
    let cells = ffi_engine.extract_cells(page_num)?;
    
    // Render page to image
    let page_image = render_pdf_page_to_image(pdf_path, page_num)?;
    
    // Run layout detection
    let layout_prediction = layout_model.predict(&page_image)?;
    
    // Match cells to detected regions
    let clusters = match_cells_to_regions(&cells, &layout_prediction.regions);
    
    // Assemble into DocItems
    let items = page_assembler.assemble(&clusters)?;
}
```

### 2. Table Structure Detection

```
Table Region → Crop Image → Table Model → Row/Col Structure → Cell Grid → TableItem
```

**Current:** Basic table detection from FFI JSON

**With ML:**
```rust
// For each detected table region
for region in layout_prediction.regions {
    if region.label == LayoutLabel::Table {
        // Crop to table bbox
        let table_image = crop_image(&page_image, &region.bbox);
        
        // Run table structure model
        let table_structure = table_model.predict(&TableInput {
            image: table_image,
            table_bbox: region.bbox,
        })?;
        
        // Match text cells to table cells
        let table_cells = cell_matcher.match_cells(
            &text_cells,
            &table_structure.cells
        );
        
        // Create TableItem
        let table_item = DocItem::Table(TableItem {
            data: build_table_data(table_structure, table_cells),
            caption: detect_caption(&surrounding_cells),
        });
    }
}
```

---

## Performance Considerations

### Model Loading (First Call)
- Layout Model: ~100-500ms load time
- Table Model: ~50-200ms load time
- **Solution:** Use lazy loading + caching (already implemented)

### Inference Time
- Layout detection: ~50-200ms per page
- Table structure: ~30-100ms per table
- **Total overhead:** ~100-300ms per page with tables

### Memory Usage
- Models in memory: ~300-500 MB
- **Solution:** Clear cache when done: `clear_model_cache()`

### Optimization Tips
1. **Batch Processing:** Process multiple pages before clearing cache
2. **Parallel Inference:** Use rayon for multi-page PDFs (future)
3. **Model Quantization:** INT8 models for 4x speed (future)

---

## Fallback Strategy

The system is designed to gracefully degrade:

```
[1] Try ML pipeline → [2] Try FFI only → [3] Try precision mode → [4] Error
```

**Current behavior:**
- Models not found → Use FFI JSON parsing (current behavior)
- FFI fails → Use precision mode (pdf_extract)
- All fail → Return error

**With ML activated:**
- Models found → Use full ML pipeline
- Model inference fails → Fall back to FFI JSON parsing
- Maintains robustness

---

## Testing Checklist

Before activating ML in production:

- [ ] Export models successfully
- [ ] Models load without errors
- [ ] Inference runs without crashes
- [ ] Output quality improves vs FFI-only
- [ ] Performance is acceptable (<500ms/page overhead)
- [ ] Memory usage is reasonable (<1GB peak)
- [ ] Fallback works when models missing
- [ ] Cache reuse works (no reload on 2nd conversion)
- [ ] Benchmark suite shows improvements

---

## Future Enhancements

### Short Term
1. ✅ Model caching (done)
2. ⏸️ Activate inference (waiting for models)
3. ⏸️ Add PDF→Image rendering
4. ⏸️ Test with real documents

### Medium Term
1. Parallel page processing
2. Batch conversion API
3. Model quantization (INT8)
4. GPU acceleration support

### Long Term
1. Custom model fine-tuning
2. Domain-specific models
3. Online learning
4. Incremental updates

---

## Troubleshooting

### Models Not Loading
```
⚠️  Model layout_model.onnx not found in any search path
```

**Solution:** Run `python scripts/export_onnx_models.py`

### ONNX Runtime Errors
```
❌ Failed to load LayoutModel: ONNX Runtime error
```

**Solutions:**
1. Check ONNX Runtime version: Should be compatible with models
2. Verify model files aren't corrupted: Run `onnx.checker.check_model()`
3. Check system requirements: CPU should support required ops

### Out of Memory
```
thread 'main' panicked at 'allocation failed'
```

**Solutions:**
1. Clear cache between conversions: `clear_model_cache()`
2. Process fewer pages at once
3. Use model quantization
4. Increase system memory

---

## References

- **Docling Models:** https://github.com/DS4SD/docling-ibm-models
- **ONNX Runtime Rust:** https://docs.rs/ort/
- **Model Export:** `transmutation/scripts/export_onnx_models.py`
- **Model Manager:** `transmutation/src/ml/model_manager.rs`
- **Model Cache:** `transmutation/src/ml/model_cache.rs`

---

## Summary

✅ **Infrastructure Complete:** All caching and loading mechanisms ready  
⏸️ **Waiting for Models:** Need to run export script  
📋 **Integration Guide:** Follow steps above to activate  
🔄 **Fallback Ready:** System works without models  

The ML pipeline is **ready to activate** as soon as ONNX models are available!

