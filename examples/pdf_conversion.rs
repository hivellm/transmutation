//! PDF conversion example
//!
//! This example demonstrates how to convert PDF files to Markdown.
//!
//! Usage:
//! ```bash
//! cargo run --example pdf_conversion --features pdf path/to/document.pdf
//! ```

use transmutation::{ConversionOptions, Converter, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get PDF path from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <pdf-file>", args[0]);
        eprintln!("Example: {} document.pdf", args[0]);
        std::process::exit(1);
    }

    let pdf_path = &args[1];
    println!("Converting PDF: {}", pdf_path);

    // Create converter
    let converter = Converter::new()?;

    // Example 1: Basic conversion to Markdown
    println!("\n=== Example 1: Basic PDF → Markdown ===");
    let result = converter
        .convert(pdf_path)
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .execute()
        .await?;

    println!("✓ Converted {} pages", result.page_count());
    println!("  Input size:  {} bytes", result.input_size());
    println!("  Output size: {} bytes", result.output_size());
    println!("  Duration:    {:?}", result.duration());
    println!(
        "  Compression: {:.1}%",
        (1.0 - result.output_size() as f64 / result.input_size() as f64) * 100.0
    );

    // Save to file
    result.save("data/output.md").await?;
    println!("  Saved to: data/output.md");

    // Example 2: Split by pages
    println!("\n=== Example 2: Split by Pages ===");
    let result = converter
        .convert(pdf_path)
        .to(OutputFormat::Markdown {
            split_pages: true,
            optimize_for_llm: true,
        })
        .execute()
        .await?;

    println!(
        "✓ Generated {} separate Markdown files",
        result.page_count()
    );
    result.save("output_pages/document.md").await?;
    println!("  Saved to: output_pages/document_page_*.md");

    // Example 3: Convert to JSON
    println!("\n=== Example 3: PDF → JSON ===");
    let result = converter
        .convert(pdf_path)
        .to(OutputFormat::Json {
            structured: true,
            include_metadata: true,
        })
        .execute()
        .await?;

    println!("✓ Converted to structured JSON");
    result.save("data/output.json").await?;
    println!("  Saved to: data/output.json");

    // Example 4: Custom options
    println!("\n=== Example 4: Custom Options ===");
    let options = ConversionOptions {
        split_pages: false,
        optimize_for_llm: true,
        preserve_layout: true,
        extract_tables: true,
        remove_headers_footers: true,
        normalize_whitespace: true,
        ..Default::default()
    };

    let result = converter
        .convert(pdf_path)
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .with_options(options)
        .execute()
        .await?;

    println!("✓ Converted with custom options");
    println!("  Pages: {}", result.page_count());
    println!("  Tables: {}", result.metadata.page_count);

    result.save("output_custom.md").await?;
    println!("  Saved to: output_custom.md");

    println!("\n✨ All examples completed successfully!");

    Ok(())
}
