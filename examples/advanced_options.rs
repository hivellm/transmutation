//! Advanced PDF conversion with custom options
//!
//! This example demonstrates advanced configuration options for document conversion.

use transmutation::{ConversionOptions, Converter, ImageQuality, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::new()?;

    // Configure conversion options
    let options = ConversionOptions {
        // Output control
        split_pages: true,
        optimize_for_llm: true,
        max_chunk_size: 1024, // Tokens per chunk

        // Quality settings
        image_quality: ImageQuality::High,
        dpi: 300,
        ocr_language: "eng".to_string(),

        // Processing options
        preserve_layout: true,
        extract_tables: true,
        extract_images: true,
        include_metadata: true,

        // Optimization
        compression_level: 9,
        remove_headers_footers: true,
        remove_watermarks: false,
        normalize_whitespace: true,
    };

    println!("Converting with advanced options...");

    // Convert with custom options
    let result = converter
        .convert("document.pdf")
        .to(OutputFormat::Markdown {
            split_pages: true,
            optimize_for_llm: true,
        })
        .with_options(options)
        .execute()
        .await?;

    println!("\n✅ Conversion complete!");

    // Display metadata if available
    if let Some(title) = &result.metadata.title {
        println!("📖 Title: {}", title);
    }
    if let Some(author) = &result.metadata.author {
        println!("👤 Author: {}", author);
    }

    println!("\n📊 Statistics:");
    println!("  Pages: {}", result.page_count());
    println!("  Tables: {}", result.statistics.tables_extracted);
    println!("  Chunks: {}", result.chunk_count());

    // Save each page separately if split_pages is enabled
    if options.split_pages {
        result.save("data/output").await?;
        println!("\n💾 Saved {} pages to data/output_page_*.md", result.page_count());
    } else {
        result.save("data/output.md").await?;
        println!("\n💾 Saved to data/output.md");
    }

    Ok(())
}


