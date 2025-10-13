#!/usr/bin/env python3
"""
Download alternative ONNX models for document layout analysis
"""

import sys
from pathlib import Path
import urllib.request
import json

def download_file(url, dest):
    """Download file with progress"""
    print(f"   📥 Downloading from {url}")
    
    def reporthook(count, block_size, total_size):
        if total_size > 0:
            percent = int(count * block_size * 100 / total_size)
            sys.stdout.write(f"\r      Progress: {percent}%")
            sys.stdout.flush()
    
    try:
        urllib.request.urlretrieve(url, dest, reporthook)
        sys.stdout.write("\n")
        return True
    except Exception as e:
        print(f"\n      ❌ Error: {e}")
        return False

def download_yolov8_document_layout():
    """Download YOLOv8 ONNX for document layout"""
    print("\n🔍 Option 1: YOLOv8 Document Layout")
    
    # YOLOv8 models converted to ONNX for document layout
    # These are trained on datasets like PubLayNet, DocLayNet
    
    models = {
        "yolov8n-doclayout.onnx": {
            "url": "https://github.com/ultralytics/assets/releases/download/v0.0.0/yolov8n.onnx",
            "desc": "YOLOv8 Nano - Fast, lower accuracy"
        },
        "yolov8s-doclayout.onnx": {
            "url": "https://github.com/ultralytics/assets/releases/download/v0.0.0/yolov8s.onnx",
            "desc": "YOLOv8 Small - Balanced"
        }
    }
    
    models_dir = Path("models")
    models_dir.mkdir(exist_ok=True)
    
    print("   ℹ️  YOLOv8 needs to be trained on document layout datasets")
    print("   ℹ️  Using base YOLOv8 as starting point")
    
    # Download a base YOLOv8 model
    # Note: This needs to be retrained on document layout data
    print("\n   ⚠️  Base YOLOv8 not trained for document layout")
    print("      Need to find pre-trained document layout model")
    
    return False

def download_layoutlmv3_onnx():
    """Try to find LayoutLMv3 in ONNX format"""
    print("\n🔍 Option 2: LayoutLMv3 (Microsoft)")
    
    try:
        from huggingface_hub import hf_hub_download, list_repo_files
        
        # Check Microsoft LayoutLMv3 repos
        repos = [
            "microsoft/layoutlmv3-base",
            "microsoft/layoutlmv3-large"
        ]
        
        for repo in repos:
            print(f"   Checking {repo}...")
            try:
                files = list_repo_files(repo)
                onnx_files = [f for f in files if f.endswith('.onnx')]
                
                if onnx_files:
                    print(f"   ✅ Found ONNX: {onnx_files}")
                    
                    # Download
                    for onnx_file in onnx_files[:1]:  # Download first one
                        local_path = hf_hub_download(repo_id=repo, filename=onnx_file)
                        
                        # Copy to models/
                        import shutil
                        dest = Path("models") / Path(onnx_file).name
                        shutil.copy(local_path, dest)
                        print(f"   ✅ Saved to: {dest}")
                        print(f"   📊 Size: {dest.stat().st_size / 1024 / 1024:.1f} MB")
                        return True
                else:
                    print(f"   ⚠️  No ONNX files in {repo}")
            except Exception as e:
                print(f"   ❌ Error: {e}")
        
        return False
        
    except ImportError:
        print("   ❌ huggingface-hub not installed")
        return False

def download_doclaynet_onnx():
    """Try to find DocLayNet model in ONNX"""
    print("\n🔍 Option 3: DocLayNet Models")
    
    try:
        from huggingface_hub import hf_hub_download, list_repo_files, HfApi
        
        api = HfApi()
        
        # Search for DocLayNet ONNX models
        print("   Searching HuggingFace for 'doclaynet onnx'...")
        
        results = api.list_models(search="doclaynet", limit=10)
        
        for model in results:
            model_id = model.id
            print(f"   Found: {model_id}")
            
            try:
                files = list_repo_files(model_id)
                onnx_files = [f for f in files if f.endswith('.onnx')]
                
                if onnx_files:
                    print(f"   ✅ Has ONNX: {onnx_files}")
                    
                    # Download first ONNX file
                    onnx_file = onnx_files[0]
                    print(f"   📥 Downloading {onnx_file}...")
                    
                    local_path = hf_hub_download(repo_id=model_id, filename=onnx_file)
                    
                    # Copy to models/
                    import shutil
                    dest = Path("models") / "doclayout_model.onnx"
                    shutil.copy(local_path, dest)
                    print(f"   ✅ Saved to: {dest}")
                    print(f"   📊 Size: {dest.stat().st_size / 1024 / 1024:.1f} MB")
                    
                    # Save metadata
                    metadata = {
                        "model_id": model_id,
                        "file": onnx_file,
                        "source": "huggingface"
                    }
                    
                    with open("models/doclayout_model.json", "w") as f:
                        json.dump(metadata, f, indent=2)
                    
                    return True
                    
            except Exception as e:
                print(f"   ⚠️  Error checking {model_id}: {e}")
                continue
        
        return False
        
    except ImportError as e:
        print(f"   ❌ Missing dependency: {e}")
        return False

def download_dit_onnx():
    """Download DiT (Document Image Transformer) ONNX"""
    print("\n🔍 Option 4: DiT (Document Image Transformer)")
    
    try:
        from huggingface_hub import hf_hub_download, list_repo_files, HfApi
        
        api = HfApi()
        
        # Search for DiT models
        print("   Searching for DiT models...")
        
        repos_to_try = [
            "microsoft/dit-base",
            "microsoft/dit-large",
        ]
        
        for repo in repos_to_try:
            print(f"   Checking {repo}...")
            try:
                files = list_repo_files(repo)
                onnx_files = [f for f in files if f.endswith('.onnx')]
                
                if onnx_files:
                    print(f"   ✅ Found ONNX: {onnx_files}")
                    
                    onnx_file = onnx_files[0]
                    print(f"   📥 Downloading {onnx_file}...")
                    
                    local_path = hf_hub_download(repo_id=repo, filename=onnx_file)
                    
                    # Copy to models/
                    import shutil
                    dest = Path("models") / "dit_layout_model.onnx"
                    shutil.copy(local_path, dest)
                    print(f"   ✅ Saved to: {dest}")
                    return True
                    
            except Exception as e:
                print(f"   ⚠️  {e}")
                continue
        
        return False
        
    except ImportError:
        print("   ❌ huggingface-hub not installed")
        return False

def search_and_download_any_onnx():
    """Search broadly for any document layout ONNX model"""
    print("\n🔍 Option 5: Broad Search for Document Layout ONNX")
    
    try:
        from huggingface_hub import HfApi, hf_hub_download, list_repo_files
        
        api = HfApi()
        
        search_terms = [
            "document layout onnx",
            "pdf layout onnx",
            "page layout onnx",
            "layout detection onnx"
        ]
        
        for search_term in search_terms:
            print(f"\n   Searching: '{search_term}'...")
            
            try:
                results = api.list_models(search=search_term, limit=20)
                
                for model in results:
                    model_id = model.id
                    
                    try:
                        files = list_repo_files(model_id)
                        onnx_files = [f for f in files if f.endswith('.onnx')]
                        
                        if onnx_files:
                            print(f"\n   ✅ FOUND: {model_id}")
                            print(f"      ONNX files: {onnx_files}")
                            
                            # Download first ONNX file
                            onnx_file = onnx_files[0]
                            print(f"      📥 Downloading {onnx_file}...")
                            
                            local_path = hf_hub_download(repo_id=model_id, filename=onnx_file)
                            
                            # Copy to models/
                            import shutil
                            dest = Path("models") / "layout_model.onnx"
                            shutil.copy(local_path, dest)
                            
                            size_mb = dest.stat().st_size / 1024 / 1024
                            print(f"      ✅ Saved to: {dest}")
                            print(f"      📊 Size: {size_mb:.1f} MB")
                            
                            # Save metadata
                            metadata = {
                                "model_id": model_id,
                                "file": onnx_file,
                                "source": "huggingface",
                                "search_term": search_term
                            }
                            
                            with open("models/layout_model.json", "w") as f:
                                json.dump(metadata, f, indent=2)
                            
                            return True
                            
                    except Exception as e:
                        continue
                        
            except Exception as e:
                print(f"   ⚠️  Search error: {e}")
                continue
        
        return False
        
    except ImportError as e:
        print(f"   ❌ Missing: {e}")
        return False

def main():
    print("╔════════════════════════════════════════╗")
    print("║  🚀 Downloading Alternative ONNX      ║")
    print("╚════════════════════════════════════════╝")
    
    # Try each option
    success = False
    
    # Option 3: DocLayNet
    if not success:
        success = download_doclaynet_onnx()
    
    # Option 4: DiT
    if not success:
        success = download_dit_onnx()
    
    # Option 2: LayoutLMv3
    if not success:
        success = download_layoutlmv3_onnx()
    
    # Option 5: Broad search
    if not success:
        success = search_and_download_any_onnx()
    
    print("\n" + "="*44)
    
    if success:
        print("✅ SUCCESS! Model downloaded to models/")
        print("\n📝 Next steps:")
        print("  1. Update Rust code to use the ONNX model")
        print("  2. Test inference")
        print("  3. Compare output with rule-based")
    else:
        print("❌ No ONNX models found")
        print("\n💡 Fallback options:")
        print("  1. Continue with rule-based (works now)")
        print("  2. Manually export docling models")
        print("  3. Train YOLOv8 on DocLayNet dataset")
    
    return 0 if success else 1

if __name__ == "__main__":
    sys.exit(main())

