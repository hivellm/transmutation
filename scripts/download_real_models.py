#!/usr/bin/env python3
"""
Download and convert docling models to ONNX for 100% Rust inference
"""

import sys
import os
from pathlib import Path

def download_and_convert_models():
    """Download docling models and convert to ONNX"""
    
    print("╔════════════════════════════════════════╗")
    print("║  📦 Downloading Docling Models        ║")
    print("╚════════════════════════════════════════╝\n")
    
    # Create models directory
    models_dir = Path("models")
    models_dir.mkdir(exist_ok=True)
    
    try:
        import torch
        import onnx
        from huggingface_hub import hf_hub_download, snapshot_download
        
        print("✅ Dependencies OK\n")
        
        # Download docling models from HuggingFace
        print("📥 Downloading docling models...")
        
        # Try to find pre-converted ONNX models or download PyTorch models
        repo_id = "ds4sd/docling-models"
        
        try:
            # Try to download the whole model repository
            print(f"   Downloading from {repo_id}...")
            local_dir = snapshot_download(
                repo_id=repo_id,
                cache_dir=".cache/huggingface",
                local_dir=".cache/docling_models"
            )
            print(f"   ✅ Downloaded to: {local_dir}\n")
            
            # List what we got
            print("📂 Contents:")
            for root, dirs, files in os.walk(local_dir):
                level = root.replace(local_dir, '').count(os.sep)
                indent = ' ' * 2 * level
                print(f'{indent}{os.path.basename(root)}/')
                subindent = ' ' * 2 * (level + 1)
                for file in files[:10]:  # Limit to first 10 files per dir
                    print(f'{subindent}{file}')
                if len(files) > 10:
                    print(f'{subindent}... and {len(files) - 10} more files')
            
            print("\n⚠️  Models downloaded but need conversion to ONNX")
            print("    Docling uses PyTorch models - we need to export them")
            
            return local_dir
            
        except Exception as e:
            print(f"   ❌ Error: {e}")
            return None
            
    except ImportError as e:
        print(f"❌ Missing dependency: {e}")
        print("\nInstall with:")
        print("  pip3 install torch onnx huggingface-hub")
        return None

if __name__ == "__main__":
    result = download_and_convert_models()
    sys.exit(0 if result else 1)

