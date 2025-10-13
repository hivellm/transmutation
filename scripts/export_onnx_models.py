#!/usr/bin/env python3
"""
Export docling models to ONNX format for Rust inference

This script exports:
1. Layout Model - Detects document regions (text, tables, figures, etc.)
2. TableFormer Model - Extracts table structure (rows, columns, cells)

Requirements:
    pip install docling torch onnx onnxruntime
    
Usage:
    python scripts/export_onnx_models.py
    
Output:
    models/layout_model.onnx
    models/table_structure_model.onnx
"""

import os
import sys
from pathlib import Path
import torch
import onnx
from onnx import shape_inference

# Add parent directory to path for imports
script_dir = Path(__file__).parent
project_root = script_dir.parent
sys.path.insert(0, str(project_root))

def export_layout_model():
    """Export layout detection model to ONNX"""
    print("\n🔄 Exporting Layout Model...")
    
    try:
        from docling_ibm_models.layoutmodel.layout_predictor import LayoutPredictor
        
        # Initialize model
        print("  📦 Loading model...")
        predictor = LayoutPredictor()
        model = predictor.model
        model.eval()
        
        # Create dummy input (batch, channels, height, width)
        # Layout model typically uses 1024x1024 images
        dummy_input = torch.randn(1, 3, 1024, 1024)
        
        # Export path
        output_path = project_root / "models" / "layout_model.onnx"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        print(f"  💾 Exporting to {output_path}...")
        
        # Export to ONNX
        torch.onnx.export(
            model,
            dummy_input,
            str(output_path),
            export_params=True,
            opset_version=14,
            do_constant_folding=True,
            input_names=['input'],
            output_names=['output'],
            dynamic_axes={
                'input': {0: 'batch_size'},
                'output': {0: 'batch_size'}
            }
        )
        
        # Verify and optimize
        print("  ✅ Verifying ONNX model...")
        onnx_model = onnx.load(str(output_path))
        onnx.checker.check_model(onnx_model)
        
        # Infer shapes
        onnx_model = shape_inference.infer_shapes(onnx_model)
        onnx.save(onnx_model, str(output_path))
        
        # Get model size
        size_mb = output_path.stat().st_size / (1024 * 1024)
        print(f"  ✅ Layout Model exported successfully ({size_mb:.1f} MB)")
        
        return True
        
    except ImportError as e:
        print(f"  ❌ Error: docling_ibm_models not installed")
        print(f"     Install with: pip install docling-ibm-models")
        return False
    except Exception as e:
        print(f"  ❌ Error exporting layout model: {e}")
        import traceback
        traceback.print_exc()
        return False

def export_table_structure_model():
    """Export TableFormer model to ONNX"""
    print("\n🔄 Exporting Table Structure Model...")
    
    try:
        from docling_ibm_models.tableformer.table_predictor import TablePredictor
        
        # Initialize model
        print("  📦 Loading model...")
        predictor = TablePredictor()
        model = predictor.model
        model.eval()
        
        # Create dummy input (batch, channels, height, width)
        # TableFormer typically uses 1024x1024 images
        dummy_input = torch.randn(1, 3, 1024, 1024)
        
        # Export path
        output_path = project_root / "models" / "table_structure_model.onnx"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        print(f"  💾 Exporting to {output_path}...")
        
        # Export to ONNX
        torch.onnx.export(
            model,
            dummy_input,
            str(output_path),
            export_params=True,
            opset_version=14,
            do_constant_folding=True,
            input_names=['input'],
            output_names=['row_logits', 'col_logits', 'cell_logits'],
            dynamic_axes={
                'input': {0: 'batch_size'},
                'row_logits': {0: 'batch_size'},
                'col_logits': {0: 'batch_size'},
                'cell_logits': {0: 'batch_size'}
            }
        )
        
        # Verify and optimize
        print("  ✅ Verifying ONNX model...")
        onnx_model = onnx.load(str(output_path))
        onnx.checker.check_model(onnx_model)
        
        # Infer shapes
        onnx_model = shape_inference.infer_shapes(onnx_model)
        onnx.save(onnx_model, str(output_path))
        
        # Get model size
        size_mb = output_path.stat().st_size / (1024 * 1024)
        print(f"  ✅ Table Structure Model exported successfully ({size_mb:.1f} MB)")
        
        return True
        
    except ImportError as e:
        print(f"  ❌ Error: docling_ibm_models not installed")
        print(f"     Install with: pip install docling-ibm-models")
        return False
    except Exception as e:
        print(f"  ❌ Error exporting table model: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    print("╔════════════════════════════════════════╗")
    print("║  📦 ONNX Model Export for Transmutation║")
    print("╚════════════════════════════════════════╝")
    
    # Check dependencies
    print("\n🔍 Checking dependencies...")
    try:
        import torch
        import onnx
        print(f"  ✅ PyTorch {torch.__version__}")
        print(f"  ✅ ONNX {onnx.__version__}")
    except ImportError as e:
        print(f"  ❌ Missing dependency: {e}")
        print("\n📦 Install required packages:")
        print("    pip install torch onnx onnxruntime")
        return 1
    
    # Export models
    success = True
    
    # Layout model
    if not export_layout_model():
        success = False
        print("\n⚠️  Layout model export failed, but continuing...")
    
    # Table structure model
    if not export_table_structure_model():
        success = False
        print("\n⚠️  Table structure model export failed, but continuing...")
    
    # Summary
    print("\n" + "═" * 44)
    if success:
        print("✅ All models exported successfully!")
        print("\n📁 Output directory: transmutation/models/")
        print("   - layout_model.onnx")
        print("   - table_structure_model.onnx")
        print("\n🚀 Next steps:")
        print("   1. Copy models to your deployment environment")
        print("   2. Run Rust code with --features docling-ffi")
        print("   3. Models will be loaded automatically from models/ directory")
        return 0
    else:
        print("⚠️  Some models failed to export")
        print("\n📝 Note:")
        print("   - Layout model is required for region detection")
        print("   - Table model is optional (only needed for table extraction)")
        print("   - The system will fall back to basic parsing if models unavailable")
        return 1

if __name__ == "__main__":
    sys.exit(main())

