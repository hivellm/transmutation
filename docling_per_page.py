#!/usr/bin/env python3
"""
Generate markdown per page using Docling for comparison
"""
import time
import json
from pathlib import Path
from docling.document_converter import DocumentConverter

def convert_docling_per_page(pdf_path, output_dir):
    """Convert PDF using Docling and save per page"""
    print("🐍 Generating per-page markdown with Docling")
    print(f"📄 Converting: {pdf_path}\n")
    
    output_dir = Path(output_dir)
    output_dir.mkdir(exist_ok=True, parents=True)
    
    start = time.time()
    
    # Initialize converter
    converter = DocumentConverter()
    
    # Convert document
    result = converter.convert(pdf_path)
    
    duration = time.time() - start
    
    # Get full markdown
    full_markdown = result.document.export_to_markdown()
    
    # Save full markdown for reference
    with open(output_dir / 'full_document.md', 'w', encoding='utf-8') as f:
        f.write(full_markdown)
    
    print(f"✅ Full document: {len(full_markdown)} bytes")
    
    # Try to get pages
    try:
        pages = result.document.pages
        print(f"📄 Total pages: {len(pages)}")
        
        # Export each page separately
        for i, page in enumerate(pages):
            page_md = page.export_to_markdown()
            page_file = output_dir / f'page_{i+1:04d}.md'
            
            with open(page_file, 'w', encoding='utf-8') as f:
                f.write(page_md)
            
            print(f"  Page {i+1}: {len(page_md)} bytes -> {page_file.name}")
        
        # Save metadata
        metadata = {
            'input': str(pdf_path),
            'pages': len(pages),
            'duration_seconds': duration,
            'full_size_bytes': len(full_markdown),
            'per_page_sizes': [len(page.export_to_markdown()) for page in pages]
        }
        
        with open(output_dir / 'docling_metadata.json', 'w') as f:
            json.dump(metadata, f, indent=2)
        
        print(f"\n✅ Docling per-page conversion complete!")
        print(f"⏱️  Duration: {duration:.2f} seconds")
        print(f"💾 Saved to: {output_dir}/")
        
    except Exception as e:
        print(f"⚠️  Could not split by pages: {e}")
        print("   Docling might not support per-page export for this PDF")

if __name__ == '__main__':
    pdf_file = 'data/1706.03762v7.pdf'
    output_directory = 'data/docling_pages'
    
    try:
        convert_docling_per_page(pdf_file, output_directory)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()

