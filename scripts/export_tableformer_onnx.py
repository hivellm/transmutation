#!/usr/bin/env python3
"""
Export docling TableFormer Model to ONNX format

This script exports both Fast and Accurate TableFormer models to ONNX
for use in the Rust implementation.

Usage:
    python scripts/export_tableformer_onnx.py --mode fast --output models/tableformer_fast.onnx
    python scripts/export_tableformer_onnx.py --mode accurate --output models/tableformer_accurate.onnx

Requirements:
    pip install docling docling-ibm-models torch onnx
"""

import argparse
import torch
import sys
from pathlib import Path

try:
    from docling_ibm_models.tableformer.data_management.tf_predictor import TFPredictor
    import docling_ibm_models.tableformer.common as c
except ImportError:
    print("ERROR: docling_ibm_models not installed")
    print("Install with: pip install docling-ibm-models")
    sys.exit(1)


def export_tableformer(output_path: Path, mode: str = "fast", model_path: str = None):
    """Export TableFormer model to ONNX format
    
    Args:
        output_path: Where to save ONNX model
        mode: "fast" or "accurate"
        model_path: Path to model artifacts (optional)
    """
    
    print(f"Loading TableFormer ({mode} mode)...")
    
    # Load config
    if model_path is None:
        # Use default path
        from docling.models.table_structure_model import TableStructureModel
        model_path = TableStructureModel.download_models() / "model_artifacts/tableformer" / mode
    
    config_path = Path(model_path) / "tm_config.json"
    tm_config = c.read_config(str(config_path))
    tm_config["model"]["save_dir"] = str(model_path)
    
    print(f"Config loaded from {config_path}")
    
    # Initialize predictor
    predictor = TFPredictor(tm_config, device="cpu", num_threads=1)
    
    print(f"Model loaded successfully")
    print(f"Exporting to ONNX: {output_path}")
    
    # Create dummy input - variable size table image
    # Note: TableFormer accepts variable-size inputs
    dummy_input = torch.randn(1, 3, 800, 600)  # Example size
    
    # Get the model
    model = predictor.model
    model.eval()
    
    # Export to ONNX
    torch.onnx.export(
        model,
        dummy_input,
        str(output_path),
        export_params=True,
        opset_version=17,
        do_constant_folding=True,
        input_names=['input'],
        output_names=['row_output', 'col_output', 'cell_output'],
        dynamic_axes={
            'input': {0: 'batch_size', 2: 'height', 3: 'width'},
            'row_output': {0: 'batch_size'},
            'col_output': {0: 'batch_size'},
            'cell_output': {0: 'batch_size'}
        }
    )
    
    print(f"✓ Model exported successfully to {output_path}")
    print(f"  Mode: {mode}")
    print(f"  Input shape: (batch, 3, H, W) - variable size")
    print(f"  Outputs: row predictions, column predictions, cell predictions")
    
    # Verify the exported model
    try:
        import onnx
        onnx_model = onnx.load(str(output_path))
        onnx.checker.check_model(onnx_model)
        print(f"✓ ONNX model verification passed")
    except Exception as e:
        print(f"⚠  Warning: Could not verify ONNX model: {e}")


def main():
    parser = argparse.ArgumentParser(description="Export TableFormer to ONNX")
    parser.add_argument(
        "--mode",
        type=str,
        choices=["fast", "accurate"],
        default="fast",
        help="Model mode (fast or accurate)"
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=None,
        help="Output ONNX file path (default: models/tableformer_{mode}.onnx)"
    )
    parser.add_argument(
        "--model-path",
        type=str,
        default=None,
        help="Path to tableformer model artifacts (optional)"
    )
    
    args = parser.parse_args()
    
    # Set default output path if not specified
    if args.output is None:
        args.output = Path(f"models/tableformer_{args.mode}.onnx")
    
    # Create output directory if needed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    
    try:
        export_tableformer(args.output, args.mode, args.model_path)
    except Exception as e:
        print(f"ERROR: Failed to export model: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

