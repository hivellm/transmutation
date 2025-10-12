//! Integration tests for PDF to Markdown conversion
//!
//! These tests verify the complete conversion pipeline from PDF to Markdown.

use std::path::PathBuf;
use transmutation::*;

/// Helper to get test fixture path
fn fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(filename)
}

#[tokio::test]
#[ignore] // Requires test fixtures
async fn test_simple_pdf_to_markdown() {
    let converter = Converter::new().expect("Failed to create converter");
    
    let result = converter
        .convert(fixture_path("simple.pdf"))
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .execute()
        .await;

    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
    
    let result = result.unwrap();
    assert_eq!(result.input_format, FileFormat::Pdf);
    assert!(result.page_count() > 0);
    assert!(result.output_size() > 0);
}

#[tokio::test]
#[ignore] // Requires test fixtures
async fn test_pdf_to_markdown_split_pages() {
    let converter = Converter::new().expect("Failed to create converter");
    
    let result = converter
        .convert(fixture_path("multi_page.pdf"))
        .to(OutputFormat::Markdown {
            split_pages: true,
            optimize_for_llm: true,
        })
        .execute()
        .await;

    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert!(result.page_count() > 1);
}

#[tokio::test]
#[ignore] // Requires test fixtures
async fn test_pdf_with_tables_to_markdown() {
    let converter = Converter::new().expect("Failed to create converter");
    
    let options = ConversionOptions {
        extract_tables: true,
        ..Default::default()
    };
    
    let result = converter
        .convert(fixture_path("with_tables.pdf"))
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .with_options(options)
        .execute()
        .await;

    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert!(result.statistics.tables_extracted > 0);
}

#[tokio::test]
#[ignore] // Requires test fixtures
async fn test_pdf_metadata_extraction() {
    let converter = Converter::new().expect("Failed to create converter");
    
    let result = converter
        .convert(fixture_path("with_metadata.pdf"))
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .execute()
        .await;

    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert!(result.metadata.title.is_some() || result.metadata.author.is_some());
}

#[tokio::test]
#[ignore] // Requires test fixtures
async fn test_pdf_to_json() {
    let converter = Converter::new().expect("Failed to create converter");
    
    let result = converter
        .convert(fixture_path("simple.pdf"))
        .to(OutputFormat::Json {
            structured: true,
            include_metadata: true,
        })
        .execute()
        .await;

    assert!(result.is_ok());
    
    let result = result.unwrap();
    let json_output = String::from_utf8_lossy(&result.content[0].data);
    assert!(json_output.contains("pages"));
}

#[tokio::test]
async fn test_invalid_pdf_file() {
    let converter = Converter::new().expect("Failed to create converter");
    
    // Try to convert a non-existent file
    let result = converter
        .convert("nonexistent.pdf")
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .execute()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_conversion_statistics() {
    // This test runs without fixtures to verify statistics structure
    let result = ConversionStatistics {
        input_size_bytes: 1024,
        output_size_bytes: 512,
        duration: std::time::Duration::from_secs(1),
        pages_processed: 5,
        tables_extracted: 2,
        images_extracted: 0,
        cache_hit: false,
    };

    assert_eq!(result.pages_processed, 5);
    assert_eq!(result.tables_extracted, 2);
    assert!(!result.cache_hit);
}

