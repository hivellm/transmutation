// Stub implementation - cross-platform
// For full functionality, use the full FFI implementation

#include "docling_ffi.h"
#include <string>
#include <cstring>
#include <stdlib.h>

// Cross-platform strdup
#ifdef _WIN32
    #define STRDUP _strdup
#else
    #define STRDUP strdup
#endif

static std::string g_error = "docling-parse FFI stub. Use full implementation for actual PDF parsing.";

extern "C" {

DoclingError docling_open_pdf(const char* pdf_path, DoclingDocumentHandle* out_handle) {
    g_error = "Windows stub: Use WSL/Linux build for full functionality";
    return DOCLING_ERROR_PARSE_FAILED;
}

DoclingError docling_close_pdf(DoclingDocumentHandle handle) {
    return DOCLING_OK;
}

DoclingError docling_get_page_count(DoclingDocumentHandle handle, int* out_count) {
    g_error = "Windows stub: Use WSL/Linux build";
    return DOCLING_ERROR_PARSE_FAILED;
}

DoclingError docling_get_page(DoclingDocumentHandle handle, int page_num, DoclingPage** out_page) {
    g_error = "Windows stub: Use WSL/Linux build";
    return DOCLING_ERROR_PARSE_FAILED;
}

DoclingError docling_free_page(DoclingPage* page) {
    return DOCLING_OK;
}

DoclingError docling_export_markdown(DoclingDocumentHandle handle, char** out_markdown) {
    g_error = "Stub: Use full FFI build for docling-parse functionality";
    *out_markdown = STRDUP(g_error.c_str());
    return DOCLING_ERROR_PARSE_FAILED;
}

DoclingError docling_free_string(char* str) {
    if (str) free(str);
    return DOCLING_OK;
}

const char* docling_get_last_error() {
    return g_error.c_str();
}

} // extern "C"
