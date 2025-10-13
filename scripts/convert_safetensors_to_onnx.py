#!/usr/bin/env python3
"""
Convert docling SafeTensors models to ONNX
"""

import sys
from pathlib import Path

def convert_tableformer_to_onnx():
    """Convert TableFormer models to ONNX"""
    
    print("╔════════════════════════════════════════╗")
    print("║  🔧 Converting Models to ONNX         ║")
    print("╚════════════════════════════════════════╝\n")
    
    try:
        import torch
        import onnx
        from safetensors.torch import load_file
        
        models_dir = Path("models")
        models_dir.mkdir(exist_ok=True)
        
        cache_dir = Path(".cache/docling_models/model_artifacts/tableformer")
        
        # Convert TableFormer Fast
        print("📦 Converting TableFormer (Fast)...")
        fast_model_path = cache_dir / "fast/tableformer_fast.safetensors"
        
        if fast_model_path.exists():
            # Load SafeTensors
            state_dict = load_file(str(fast_model_path))
            print(f"   ✅ Loaded {len(state_dict)} tensors")
            
            # Note: We need the model architecture to export to ONNX
            # SafeTensors only contains weights, not architecture
            print("   ⚠️  Need model architecture to export ONNX")
            print("      SafeTensors only contains weights\n")
        else:
            print(f"   ❌ Not found: {fast_model_path}\n")
        
        # The problem: SafeTensors is just weights, we need the model class
        print("❌ Cannot convert SafeTensors → ONNX without model architecture\n")
        print("📝 Solutions:")
        print("  1. Use docling Python library directly (already works)")
        print("  2. Find pre-converted ONNX models")
        print("  3. Implement models in Rust from scratch")
        print("  4. Use alternative models (YOLOv8, etc) in ONNX format\n")
        
        # Let's try solution 4: Download YOLOv8 ONNX for document layout
        print("🔄 Trying alternative: YOLOv8 Document Layout ONNX...")
        print("   Searching for pre-trained document layout ONNX models...")
        
        # Check if there are publically available ONNX models
        from huggingface_hub import hf_hub_download, list_repo_files
        
        # Try to find ONNX models in popular repos
        repos_to_try = [
            "microsoft/table-transformer-detection",
            "microsoft/table-transformer-structure-recognition",
        ]
        
        for repo in repos_to_try:
            try:
                print(f"\n   Checking {repo}...")
                files = list_repo_files(repo)
                onnx_files = [f for f in files if f.endswith('.onnx')]
                
                if onnx_files:
                    print(f"   ✅ Found ONNX models: {onnx_files}")
                    
                    # Download the ONNX model
                    for onnx_file in onnx_files:
                        local_path = hf_hub_download(
                            repo_id=repo,
                            filename=onnx_file,
                            cache_dir=".cache/huggingface"
                        )
                        
                        # Copy to models/
                        import shutil
                        dest = models_dir / Path(onnx_file).name
                        shutil.copy(local_path, dest)
                        print(f"   ✅ Saved to: {dest}")
                else:
                    print(f"   ⚠️  No ONNX files found")
                    
            except Exception as e:
                print(f"   ❌ Error: {e}")
        
        return True
        
    except ImportError as e:
        print(f"❌ Missing dependency: {e}")
        print("\nInstall with:")
        print("  pip3 install torch safetensors huggingface-hub")
        return False

if __name__ == "__main__":
    result = convert_tableformer_to_onnx()
    sys.exit(0 if result else 1)

