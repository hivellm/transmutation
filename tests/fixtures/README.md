# Test Fixtures

This directory contains test files for integration testing.

## Required Test Files

To run integration tests, place the following files here:

### PDF Files
- `sample.pdf` - Simple text PDF (1-2 pages)
- `multipage.pdf` - Multi-page document (10+ pages)
- `tables.pdf` - PDF with tables
- `scanned.pdf` - Scanned PDF for OCR testing (optional)

### DOCX Files (for future tests)
- `sample.docx` - Simple Word document
- `formatted.docx` - Document with formatting

### XLSX Files (for future tests)
- `sample.xlsx` - Simple spreadsheet
- `multishee.xlsx` - Multiple sheets

## Running Tests

```bash
# Run all integration tests (with fixtures)
cargo test --features pdf -- --ignored

# Run specific test
cargo test --features pdf test_pdf_to_markdown -- --ignored
```

## Generating Test Files

You can generate simple test PDFs using online tools or:

```python
# Using Python (if available)
from reportlab.pdfgen import canvas

c = canvas.Canvas("sample.pdf")
c.drawString(100, 750, "Hello, this is a test PDF")
c.drawString(100, 730, "Page 1 content")
c.showPage()
c.drawString(100, 750, "Page 2 content")
c.save()
```

## Note

Test fixtures are **not included in the repository** (.gitignore).
Each developer must provide their own test files.










