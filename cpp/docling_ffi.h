/*
 * C FFI API for docling-parse integration with Rust
 * Pure C interface - no C++ exposed to Rust
 */

#ifndef DOCLING_FFI_H
#define DOCLING_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

// Opaque handle to PDF document
typedef void* DoclingDocumentHandle;

// Text cell structure (C-compatible)
typedef struct {
    double x;
    double y;
    double width;
    double height;
    double font_size;
    const char* text;        // UTF-8 encoded
    const char* font_name;
} DoclingTextCell;

// Page structure
typedef struct {
    int page_number;
    double width;
    double height;
    DoclingTextCell* cells;
    size_t cell_count;
} DoclingPage;

// Error codes
typedef enum {
    DOCLING_OK = 0,
    DOCLING_ERROR_FILE_NOT_FOUND = 1,
    DOCLING_ERROR_INVALID_PDF = 2,
    DOCLING_ERROR_PARSE_FAILED = 3,
    DOCLING_ERROR_OUT_OF_MEMORY = 4,
    DOCLING_ERROR_INVALID_PAGE = 5,
} DoclingError;

// Core API Functions
DoclingError docling_open_pdf(const char* pdf_path, DoclingDocumentHandle* out_handle);
DoclingError docling_close_pdf(DoclingDocumentHandle handle);

DoclingError docling_get_page_count(DoclingDocumentHandle handle, int* out_count);
DoclingError docling_get_page(DoclingDocumentHandle handle, int page_num, DoclingPage** out_page);
DoclingError docling_free_page(DoclingPage* page);

// Export to markdown
DoclingError docling_export_markdown(DoclingDocumentHandle handle, char** out_markdown);
DoclingError docling_free_string(char* str);

// Get last error message
const char* docling_get_last_error();

#ifdef __cplusplus
}
#endif

#endif // DOCLING_FFI_H

