#!/usr/bin/env python3
"""Search for document layout ONNX models on HuggingFace"""

from huggingface_hub import HfApi, hf_hub_download
from pathlib import Path

print("🔍 Searching for ONNX document layout models...")

api = HfApi()

# Search for models
searches = [
    "document layout onnx",
    "table detection onnx",
    "pdf layout onnx"
]

found_models = []

for search_term in searches:
    print(f"\n📋 Searching: '{search_term}'...")
    try:
        results = api.list_models(search=search_term, limit=5)
        for model in results:
            model_id = model.id
            print(f"   - {model_id}")
            found_models.append(model_id)
    except Exception as e:
        print(f"   Error: {e}")

# Download one if we found any
if found_models:
    print(f"\n📥 Trying to download first model: {found_models[0]}")
    # We would download here
    
print("\n✅ Done")

