# ML Model Export Scripts

These scripts export docling's PyTorch models to ONNX format for use in the Rust implementation.

## Prerequisites

```bash
pip install docling docling-ibm-models torch onnx
```

## Export Layout Model

```bash
python scripts/export_layout_model_onnx.py --output models/layout_model.onnx
```

This will:
1. Download the layout model from HuggingFace (if needed)
2. Load the model in CPU mode
3. Export to ONNX format with opset 17
4. Save to `models/layout_model.onnx`

**Model Details:**
- Input: `(batch, 3, 1025, 1025)` - RGB image
- Output: Segmentation masks for each document element type
- Classes: Text, Title, Section-header, List-item, Caption, Footnote, Page-header, Page-footer, Table, Figure, Formula, Code

## Export TableFormer Models

### Fast Mode (Faster inference, slightly lower accuracy)
```bash
python scripts/export_tableformer_onnx.py --mode fast --output models/tableformer_fast.onnx
```

### Accurate Mode (Higher accuracy, slower inference)
```bash
python scripts/export_tableformer_onnx.py --mode accurate --output models/tableformer_accurate.onnx
```

**Model Details:**
- Input: `(batch, 3, H, W)` - Variable size table region image
- Outputs: Row predictions, Column predictions, Cell predictions
- Used to extract table structure (rows, columns, cells, spans)

## Using Exported Models in Rust

After exporting, the models will be automatically discovered by the Rust code:

```rust
use transmutation::ml::{LayoutModel, TableStructureModel};

// Load models
let layout_model = LayoutModel::new("models/layout_model.onnx")?;
let table_model = TableStructureModel::new("models/tableformer_fast.onnx", 2.0)?;

// Use in pipeline
let layout_pred = layout_model.predict(&page_image)?;
let table_struct = table_model.predict(&table_input)?;
```

## Troubleshooting

### Model download fails
The scripts will automatically download models from HuggingFace. If this fails:
1. Check internet connection
2. Manually download from: https://huggingface.co/ds4sd/docling-models
3. Pass `--model-path` to the export scripts

### ONNX export fails
Make sure you have compatible versions:
```bash
pip install --upgrade torch onnx
```

### Out of memory
Use CPU mode (default) if GPU memory is limited. The export process is memory-intensive but only needs to be done once.

