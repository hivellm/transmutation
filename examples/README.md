# Transmutation Examples

This directory contains examples demonstrating how to use Transmutation.

## Available Examples

### PDF Conversion (`pdf_conversion.rs`)
Demonstrates PDF to Markdown/JSON conversion.

```bash
# Run with a PDF file
cargo run --example pdf_conversion --features pdf path/to/document.pdf
```

Features demonstrated:
- Basic PDF → Markdown
- Split by pages
- PDF → JSON
- Custom options

## Coming Soon

- `batch_processing.rs` - Batch convert multiple files
- `vectorizer_integration.rs` - Integration with Vectorizer
- `custom_pipeline.rs` - Custom conversion pipelines
- `docx_conversion.rs` - Word document conversion
- `xlsx_conversion.rs` - Excel spreadsheet conversion

## Running Examples

Make sure to build with the appropriate features:

```bash
# PDF examples
cargo run --example pdf_conversion --features pdf file.pdf

# Office examples (coming soon)
cargo run --example docx_conversion --features office file.docx

# All features
cargo run --example pdf_conversion --features full file.pdf
```








