//! Integration tests for PDF conversion
//!
//! These tests require actual PDF files in tests/fixtures/

#[cfg(feature = "pdf")]
mod pdf_tests {
    use transmutation::{Converter, ConversionOptions, OutputFormat};

    #[tokio::test]
    #[ignore] // Ignored by default - requires test PDF files
    async fn test_pdf_to_markdown() {
        let converter = Converter::new().unwrap();
        
        let result = converter
            .convert("tests/fixtures/sample.pdf")
            .to(OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            })
            .execute()
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.page_count() > 0);
        assert!(result.output_size() > 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_pdf_split_pages() {
        let converter = Converter::new().unwrap();
        
        let result = converter
            .convert("tests/fixtures/sample.pdf")
            .to(OutputFormat::Markdown {
                split_pages: true,
                optimize_for_llm: true,
            })
            .execute()
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should have multiple outputs (one per page)
        assert_eq!(result.content.len(), result.page_count());
    }

    #[tokio::test]
    #[ignore]
    async fn test_pdf_to_json() {
        let converter = Converter::new().unwrap();
        
        let result = converter
            .convert("tests/fixtures/sample.pdf")
            .to(OutputFormat::Json {
                structured: true,
                include_metadata: true,
            })
            .execute()
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should have single JSON output
        assert_eq!(result.content.len(), 1);
        
        // Verify JSON is valid
        let json_str = String::from_utf8(result.content[0].data.clone()).unwrap();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(json["metadata"].is_object());
        assert!(json["pages"].is_array());
    }

    #[tokio::test]
    #[ignore]
    async fn test_pdf_metadata_extraction() {
        let converter = Converter::new().unwrap();
        
        let result = converter
            .convert("tests/fixtures/sample.pdf")
            .to(OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: false,
            })
            .execute()
            .await
            .unwrap();

        // Check metadata is extracted
        assert!(result.metadata.page_count > 0);
        // Title and author may be None for some PDFs
    }

    #[tokio::test]
    #[ignore]
    async fn test_pdf_optimization() {
        let converter = Converter::new().unwrap();
        
        // Without optimization
        let result_raw = converter
            .convert("tests/fixtures/sample.pdf")
            .to(OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: false,
            })
            .execute()
            .await
            .unwrap();

        // With optimization
        let result_opt = converter
            .convert("tests/fixtures/sample.pdf")
            .to(OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            })
            .execute()
            .await
            .unwrap();

        // Optimized should be smaller or equal
        assert!(result_opt.output_size() <= result_raw.output_size());
    }
}

// Note: To run these tests, create tests/fixtures/ directory
// and add sample PDF files, then run:
// cargo test --features pdf -- --ignored








