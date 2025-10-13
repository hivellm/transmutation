#!/usr/bin/env python3
"""Test docling models and see what's available"""

from docling.document_converter import DocumentConverter

print("Creating converter...")
converter = DocumentConverter()

print("\nConverter attributes:")
for attr in dir(converter):
    if not attr.startswith('_'):
        print(f"  - {attr}")

# Try to access models if they exist
if hasattr(converter, 'pipeline'):
    print("\nPipeline:")
    print(f"  {converter.pipeline}")

print("\nDone")

