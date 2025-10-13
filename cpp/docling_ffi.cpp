/*
 * C++ implementation of FFI wrapper for docling-parse
 * Bridges C API to docling-parse C++ code
 */

#include "docling_ffi.h"
#include <v2.h>
#include <resources.h>
#include <string>
#include <memory>
#include <cstring>
#include <fstream>
#include <filesystem>

// Thread-local error message
thread_local std::string g_last_error;

// Internal document structure
struct DoclingDocument {
    std::string pdf_path;
    nlohmann::json config;
    nlohmann::json result;
};

extern "C" {

void set_last_error(const std::string& msg) {
    g_last_error = msg;
}

const char* docling_get_last_error() {
    return g_last_error.c_str();
}

DoclingError docling_open_pdf(const char* pdf_path, DoclingDocumentHandle* out_handle) {
    std::cerr << "[FFI] docling_open_pdf called with: " << (pdf_path ? pdf_path : "NULL") << std::endl;
    
    if (!pdf_path || !out_handle) {
        set_last_error("Invalid arguments");
        std::cerr << "[FFI] ERROR: Invalid arguments" << std::endl;
        return DOCLING_ERROR_INVALID_PDF;
    }

    try {
        auto doc = new DoclingDocument();
        doc->pdf_path = pdf_path;
        
        // Initialize docling-parse config
        doc->config["data"] = nlohmann::json::object();
        doc->config["files"]["pdf"]["filename"] = pdf_path;
        
        *out_handle = doc;
        std::cerr << "[FFI] PDF opened successfully" << std::endl;
        return DOCLING_OK;
        
    } catch (const std::exception& e) {
        set_last_error(std::string("Failed to open PDF: ") + e.what());
        return DOCLING_ERROR_PARSE_FAILED;
    }
}

DoclingError docling_close_pdf(DoclingDocumentHandle handle) {
    if (!handle) {
        return DOCLING_ERROR_INVALID_PDF;
    }
    
    try {
        auto doc = static_cast<DoclingDocument*>(handle);
        delete doc;
        return DOCLING_OK;
    } catch (const std::exception& e) {
        set_last_error(std::string("Failed to close PDF: ") + e.what());
        return DOCLING_ERROR_PARSE_FAILED;
    }
}

DoclingError docling_get_page_count(DoclingDocumentHandle handle, int* out_count) {
    if (!handle || !out_count) {
        return DOCLING_ERROR_INVALID_PDF;
    }
    
    try {
        auto doc = static_cast<DoclingDocument*>(handle);
        
        // Parse PDF to get page count (mock for now)
        *out_count = 1; // TODO: Implement page count extraction
        return DOCLING_OK;
    } catch (const std::exception& e) {
        set_last_error(std::string("Failed to get page count: ") + e.what());
        return DOCLING_ERROR_PARSE_FAILED;
    }
}

DoclingError docling_get_page(DoclingDocumentHandle handle, int page_num, DoclingPage** out_page) {
    if (!handle || !out_page) {
        return DOCLING_ERROR_INVALID_PDF;
    }
    
    try {
        // Allocate page structure
        DoclingPage* page = (DoclingPage*)malloc(sizeof(DoclingPage));
        if (!page) {
            return DOCLING_ERROR_PARSE_FAILED;
        }

        page->page_number = page_num;
        page->width = 612.0f;  // Default US Letter width
        page->height = 792.0f; // Default US Letter height
        page->cell_count = 0;
        page->cells = nullptr;

        // TODO: Parse PDF and extract cells for this page

        *out_page = page;
        return DOCLING_OK;
    } catch (const std::exception& e) {
        set_last_error(std::string("Failed to get page: ") + e.what());
        return DOCLING_ERROR_PARSE_FAILED;
    }
}

DoclingError docling_free_page(DoclingPage* page) {
    if (!page) return DOCLING_OK;
    
    if (page->cells) {
        for (size_t i = 0; i < page->cell_count; ++i) {
            free((void*)page->cells[i].text);
            free((void*)page->cells[i].font_name);
        }
        free(page->cells);
    }
    free(page);
    return DOCLING_OK;
}

DoclingError docling_export_markdown(DoclingDocumentHandle handle, char** out_markdown) {
    if (!handle || !out_markdown) {
        return DOCLING_ERROR_INVALID_PDF;
    }
    
    try {
        auto doc = static_cast<DoclingDocument*>(handle);
        
        // Set the resources directory BEFORE creating the parser
        std::filesystem::path root_path(ROOT_PATH);
        std::filesystem::path resources_path = root_path / "docling_parse" / "pdf_resources_v2";
        resources_path = std::filesystem::absolute(resources_path);
        
        if (!std::filesystem::exists(resources_path)) {
            std::cerr << "[FFI] ERROR: Resources path does not exist: " << resources_path << std::endl;
            set_last_error("Resources path does not exist: " + resources_path.string());
            return DOCLING_ERROR_PARSE_FAILED;
        }
        
        // Set the resources directory globally using resource_utils
        std::cerr << "[FFI] Setting resources directory: " << resources_path << std::endl;
        bool set_result = resource_utils::set_resources_v2_dir(resources_path);
        std::cerr << "[FFI] set_resources_v2_dir result: " << (set_result ? "SUCCESS" : "FAILED") << std::endl;
        
        // Verify it was set correctly
        auto current_resources = resource_utils::get_resources_v2_dir(false);
        std::cerr << "[FFI] Current resources directory: " << current_resources << std::endl;
        
        if (!set_result) {
            std::cerr << "[FFI] ERROR: Failed to set resources directory" << std::endl;
            set_last_error("Failed to set resources directory");
            return DOCLING_ERROR_PARSE_FAILED;
        }
        
        // Parse PDF with docling-parse
        plib::parser parser("error");
        
        // Create temporary output file for JSON result
        std::string json_output = doc->pdf_path + ".json";
        doc->config["files"]["pdf"]["filename"] = doc->pdf_path;
        doc->config["files"]["pdf"]["output"] = json_output;
        doc->config["pdf_resource_directory"] = resources_path.string();
        
        // Log configuration
        std::cerr << "[FFI] Parsing " << doc->pdf_path << " -> " << json_output << std::endl;
        
        parser.parse(doc->config, false);
        
        std::cerr << "[FFI] Parse completed" << std::endl;
        
        // Read the JSON output
        std::ifstream json_file(json_output);
        nlohmann::json result;
        if (!json_file.is_open()) {
            set_last_error("Failed to open JSON output");
            return DOCLING_ERROR_PARSE_FAILED;
        }
        
        json_file >> result;
        json_file.close();
        
        std::cerr << "[FFI] JSON loaded successfully" << std::endl;
        
        // Return the full JSON string for Rust to parse
        // Rust has better tools for parsing complex JSON structures
        std::string json_str = result.dump();
        std::cerr << "[FFI] Returning JSON string, size: " << json_str.length() << " bytes" << std::endl;
        
        *out_markdown = strdup(json_str.c_str());
        return DOCLING_OK;
    } catch (const std::exception& e) {
        set_last_error(std::string("Failed to export markdown: ") + e.what());
        return DOCLING_ERROR_PARSE_FAILED;
    }
}

DoclingError docling_free_string(char* str) {
    if (str) {
        free(str);
    }
    return DOCLING_OK;
}

} // extern "C"
