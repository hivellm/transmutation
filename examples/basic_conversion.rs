//! Basic PDF to Markdown conversion example
//!
//! This example demonstrates how to convert a PDF file to Markdown format
//! using the Transmutation library.

use transmutation::{Converter, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the converter
    let converter = Converter::new()?;

    println!("Converting PDF to Markdown...");

    // Convert PDF to Markdown
    let result = converter
        .convert("document.pdf") // Replace with your PDF file
        .to(OutputFormat::Markdown {
            split_pages: false,     // Combine all pages into one document
            optimize_for_llm: true, // Optimize output for LLM processing
        })
        .execute()
        .await?;

    // Display conversion statistics
    println!("\n✅ Conversion complete!");
    println!("📄 Input file: {:?}", result.input_path);
    println!("📝 Pages processed: {}", result.statistics.pages_processed);
    println!("📊 Tables extracted: {}", result.statistics.tables_extracted);
    println!("⏱️  Duration: {:?}", result.statistics.duration);
    println!(
        "📏 Input size: {} bytes",
        result.statistics.input_size_bytes
    );
    println!(
        "📏 Output size: {} bytes",
        result.statistics.output_size_bytes
    );

    // Save the output
    result.save("data/output.md").await?;
    println!("\n💾 Saved to: data/output.md");

    Ok(())
}


