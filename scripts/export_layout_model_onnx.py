#!/usr/bin/env python3
"""
Export docling Layout Model to ONNX format

This script exports the Layout Model from docling_ibm_models to ONNX
for use in the Rust implementation.

Usage:
    python scripts/export_layout_model_onnx.py --output models/layout_model.onnx

Requirements:
    pip install docling docling-ibm-models torch onnx
"""

import argparse
import torch
import sys
from pathlib import Path

try:
    from docling_ibm_models.layoutmodel.layout_predictor import LayoutPredictor
except ImportError:
    print("ERROR: docling_ibm_models not installed")
    print("Install with: pip install docling-ibm-models")
    sys.exit(1)


def export_layout_model(output_path: Path, model_path: str = None):
    """Export layout model to ONNX format"""
    
    print(f"Loading LayoutPredictor...")
    
    # Initialize predictor with default or custom model path
    predictor = LayoutPredictor(
        artifact_path=model_path,
        device="cpu"  # Use CPU for export
    )
    
    print(f"Model loaded successfully")
    print(f"Exporting to ONNX: {output_path}")
    
    # Create dummy input (1, 3, 1025, 1025)
    dummy_input = torch.randn(1, 3, 1025, 1025)
    
    # Get the actual model from predictor
    model = predictor.model
    model.eval()
    
    # Export to ONNX
    torch.onnx.export(
        model,
        dummy_input,
        str(output_path),
        export_params=True,
        opset_version=17,  # Use latest stable opset
        do_constant_folding=True,
        input_names=['input'],
        output_names=['output'],
        dynamic_axes={
            'input': {0: 'batch_size'},
            'output': {0: 'batch_size'}
        }
    )
    
    print(f"✓ Model exported successfully to {output_path}")
    print(f"  Input shape: (batch, 3, 1025, 1025)")
    print(f"  Output: Segmentation masks for each class")
    
    # Verify the exported model
    try:
        import onnx
        onnx_model = onnx.load(str(output_path))
        onnx.checker.check_model(onnx_model)
        print(f"✓ ONNX model verification passed")
    except Exception as e:
        print(f"⚠  Warning: Could not verify ONNX model: {e}")


def main():
    parser = argparse.ArgumentParser(description="Export Layout Model to ONNX")
    parser.add_argument(
        "--output",
        type=Path,
        default=Path("models/layout_model.onnx"),
        help="Output ONNX file path"
    )
    parser.add_argument(
        "--model-path",
        type=str,
        default=None,
        help="Path to layout model artifacts (optional, uses default if not specified)"
    )
    
    args = parser.parse_args()
    
    # Create output directory if needed
    args.output.parent.mkdir(parents=True, exist_ok=True)
    
    try:
        export_layout_model(args.output, args.model_path)
    except Exception as e:
        print(f"ERROR: Failed to export model: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

