#!/usr/bin/env python3
"""
Compare Docling vs Transmutation performance
"""
import time
import json
from pathlib import Path
from docling.document_converter import DocumentConverter

def convert_with_docling(pdf_path, output_path):
    """Convert PDF using Docling"""
    print("🐍 Testing Docling (Python)")
    print(f"📄 Converting: {pdf_path}\n")
    
    start = time.time()
    
    # Initialize converter
    converter = DocumentConverter()
    
    # Convert document
    result = converter.convert(pdf_path)
    
    # Export to markdown
    markdown = result.document.export_to_markdown()
    
    duration = time.time() - start
    
    # Save output
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(markdown)
    
    # Get statistics
    output_size = len(markdown.encode('utf-8'))
    input_size = Path(pdf_path).stat().st_size
    
    print("✅ Docling Conversion Complete!\n")
    print("📊 Statistics:")
    print(f"  ⏱️  Duration: {duration:.2f} seconds")
    print(f"  📏 Input: {input_size / 1_000_000:.2f} MB")
    print(f"  📏 Output: {output_size / 1_000_000:.2f} MB")
    print(f"  🗜️  Compression: {input_size / output_size:.1f}x")
    
    # Try to get page count
    try:
        pages = len(result.document.pages)
        print(f"  📄 Pages: {pages}")
        print(f"  ⚡ Speed: {pages / duration:.2f} pages/sec")
    except:
        pass
    
    return {
        'duration': duration,
        'input_size': input_size,
        'output_size': output_size,
        'output_path': str(output_path)
    }

if __name__ == '__main__':
    pdf_file = 'data/1706.03762v7.pdf'
    output_file = 'data/output_docling.md'
    
    try:
        stats = convert_with_docling(pdf_file, output_file)
        
        # Save stats
        with open('data/docling_stats.json', 'w') as f:
            json.dump(stats, f, indent=2)
        
        print(f"\n💾 Saved to: {output_file}")
        print(f"📊 Stats saved to: data/docling_stats.json")
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()

