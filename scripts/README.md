# Scripts

This directory contains utility scripts for the Transmutation project.

## export_onnx_models.py

Exports docling ML models to ONNX format for use in Rust inference.

### Requirements

```bash
pip install docling torch onnx onnxruntime
```

### Usage

```bash
cd transmutation
python scripts/export_onnx_models.py
```

### Output

Creates ONNX models in `transmutation/models/`:
- `layout_model.onnx` - Document region detection (text, tables, figures)
- `table_structure_model.onnx` - Table structure extraction (rows, columns, cells)

### Troubleshooting

**Import Error:** If you get import errors, make sure docling is installed:
```bash
pip install docling docling-ibm-models
```

**CUDA/GPU Issues:** The script will use CPU by default. For GPU export:
```bash
export CUDA_VISIBLE_DEVICES=0
python scripts/export_onnx_models.py
```

**Model Size:** Exported models are typically:
- Layout Model: ~100-200 MB
- Table Model: ~50-100 MB

### Integration

Once exported, the Rust code will automatically:
1. Look for models in `transmutation/models/`
2. Fall back to system cache (`~/.cache/transmutation_models/`)
3. Use basic parsing if models unavailable

No manual configuration required!

