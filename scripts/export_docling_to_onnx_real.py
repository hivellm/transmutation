#!/usr/bin/env python3
"""
Export docling models to ONNX using the actual library
"""

import sys
import torch
from pathlib import Path

def export_layout_model():
    """Export layout model to ONNX"""
    print("\n🔧 Exporting Layout Model...")
    
    try:
        # Import docling layout model
        from docling_core.transforms.page import LayoutPredictor
        from docling_ibm_models.layoutmodel.layout_predictor import LayoutPredictor as IBMLayoutPredictor
        
        print("   📦 Loading model...")
        
        # Initialize predictor (will download model if needed)
        predictor = IBMLayoutPredictor()
        model = predictor.model
        model.eval()
        
        print("   ✅ Model loaded")
        
        # Create dummy input
        dummy_input = torch.randn(1, 3, 1024, 1024)
        
        # Export to ONNX
        output_path = Path("models/layout_model.onnx")
        output_path.parent.mkdir(exist_ok=True)
        
        print("   🔄 Exporting to ONNX...")
        torch.onnx.export(
            model,
            dummy_input,
            str(output_path),
            input_names=["input"],
            output_names=["output"],
            dynamic_axes={"input": {0: "batch_size"}, "output": {0: "batch_size"}},
            opset_version=14,
            do_constant_folding=True,
        )
        
        print(f"   ✅ Exported to: {output_path}")
        print(f"   📊 Size: {output_path.stat().st_size / 1024 / 1024:.1f} MB")
        
        return True
        
    except Exception as e:
        print(f"   ❌ Error: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    print("╔════════════════════════════════════════╗")
    print("║  🚀 Exporting Docling Models to ONNX ║")
    print("╚════════════════════════════════════════╝")
    
    # Check dependencies
    try:
        import torch
        import onnx
        import docling_ibm_models
        print("\n✅ Dependencies OK")
    except ImportError as e:
        print(f"\n❌ Missing: {e}")
        print("\nInstall:")
        print("  pip3 install torch onnx docling docling-ibm-models")
        return 1
    
    # Export layout model
    layout_ok = export_layout_model()
    
    print("\n" + "="*44)
    if layout_ok:
        print("✅ Export successful!")
        print("\n📂 Models saved to: models/")
        print("   - layout_model.onnx")
    else:
        print("❌ Export failed")
        print("\n💡 Alternative: Use rule-based layout detection")
        print("   The current implementation already works!")
    
    return 0 if layout_ok else 1

if __name__ == "__main__":
    sys.exit(main())

