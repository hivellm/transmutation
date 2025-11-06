//! Rust FFI bindings to docling-parse C++ library
//!
//! Provides direct access to docling-parse without Python dependency

#![allow(dead_code, unsafe_code, missing_docs)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::path::Path;
use std::ptr;

use crate::{Result, TransmutationError};

/// Text cell from docling-parse
#[repr(C)]
#[derive(Debug)]
pub struct DoclingTextCell {
    pub x: c_double,
    pub y: c_double,
    pub width: c_double,
    pub height: c_double,
    pub font_size: c_double,
    pub text: *const c_char,
    pub font_name: *const c_char,
}

/// Page structure
#[repr(C)]
#[derive(Debug)]
pub struct DoclingPage {
    pub page_number: c_int,
    pub width: c_double,
    pub height: c_double,
    pub cells: *mut DoclingTextCell,
    pub cell_count: usize,
}

/// Error codes
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum DoclingError {
    Ok = 0,
    FileNotFound = 1,
    InvalidPdf = 2,
    ParseFailed = 3,
    OutOfMemory = 4,
    InvalidPage = 5,
}

type DoclingDocumentHandle = *mut std::ffi::c_void;

// FFI function declarations
#[cfg(feature = "docling-ffi")]
unsafe extern "C" {
    fn docling_open_pdf(
        pdf_path: *const c_char,
        out_handle: *mut DoclingDocumentHandle,
    ) -> DoclingError;
    fn docling_close_pdf(handle: DoclingDocumentHandle) -> DoclingError;
    fn docling_get_page_count(handle: DoclingDocumentHandle, out_count: *mut c_int)
    -> DoclingError;
    fn docling_get_page(
        handle: DoclingDocumentHandle,
        page_num: c_int,
        out_page: *mut *mut DoclingPage,
    ) -> DoclingError;
    fn docling_free_page(page: *mut DoclingPage) -> DoclingError;
    fn docling_export_markdown(
        handle: DoclingDocumentHandle,
        out_markdown: *mut *mut c_char,
    ) -> DoclingError;
    fn docling_free_string(str: *mut c_char) -> DoclingError;
    fn docling_get_last_error() -> *const c_char;
}

/// Rust wrapper for docling-parse
#[derive(Debug)]
pub struct DoclingParseEngine {
    #[cfg(feature = "docling-ffi")]
    handle: DoclingDocumentHandle,
    #[cfg(not(feature = "docling-ffi"))]
    _phantom: std::marker::PhantomData<()>,
}

impl DoclingParseEngine {
    /// Open a PDF file using docling-parse
    pub fn open(path: &Path) -> Result<Self> {
        #[cfg(feature = "docling-ffi")]
        {
            let path_str = path
                .to_str()
                .ok_or_else(|| TransmutationError::conversion_failed("Invalid file path"))?;

            let c_path = CString::new(path_str).map_err(|e| {
                TransmutationError::engine_error("docling-parse", format!("Invalid path: {e}"))
            })?;

            let mut handle: DoclingDocumentHandle = ptr::null_mut();

            unsafe {
                let result = docling_open_pdf(c_path.as_ptr(), &mut handle);

                if result != DoclingError::Ok {
                    let err_msg = CStr::from_ptr(docling_get_last_error())
                        .to_string_lossy()
                        .to_string();
                    return Err(TransmutationError::engine_error("docling-parse", err_msg));
                }
            }

            Ok(Self { handle })
        }

        #[cfg(not(feature = "docling-ffi"))]
        {
            let _ = path;
            Err(TransmutationError::engine_error(
                "docling-parse",
                "docling-ffi feature not enabled. Compile with --features docling-ffi",
            ))
        }
    }

    /// Get number of pages
    pub fn page_count(&self) -> Result<usize> {
        #[cfg(feature = "docling-ffi")]
        {
            let mut count: c_int = 0;

            unsafe {
                let result = docling_get_page_count(self.handle, &mut count);
                if result != DoclingError::Ok {
                    let err_msg = CStr::from_ptr(docling_get_last_error())
                        .to_string_lossy()
                        .to_string();
                    return Err(TransmutationError::engine_error("docling-parse", err_msg));
                }
            }

            Ok(count as usize)
        }

        #[cfg(not(feature = "docling-ffi"))]
        {
            Err(TransmutationError::engine_error(
                "docling-parse",
                "Feature not enabled",
            ))
        }
    }

    /// Export to Markdown
    pub fn export_markdown(&self) -> Result<String> {
        #[cfg(feature = "docling-ffi")]
        {
            let mut markdown_ptr: *mut c_char = ptr::null_mut();

            unsafe {
                let result = docling_export_markdown(self.handle, &mut markdown_ptr);
                if result != DoclingError::Ok {
                    let err_msg = CStr::from_ptr(docling_get_last_error())
                        .to_string_lossy()
                        .to_string();
                    return Err(TransmutationError::engine_error("docling-parse", err_msg));
                }

                let markdown = CStr::from_ptr(markdown_ptr).to_string_lossy().to_string();

                docling_free_string(markdown_ptr);

                Ok(markdown)
            }
        }

        #[cfg(not(feature = "docling-ffi"))]
        {
            Err(TransmutationError::engine_error(
                "docling-parse",
                "Feature not enabled",
            ))
        }
    }
}

impl Drop for DoclingParseEngine {
    fn drop(&mut self) {
        #[cfg(feature = "docling-ffi")]
        unsafe {
            docling_close_pdf(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "docling-ffi")]
    fn test_open_pdf() {
        let result = DoclingParseEngine::open(Path::new("data/1706.03762v7.pdf"));
        assert!(result.is_ok());
    }
}
