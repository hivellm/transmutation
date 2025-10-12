// Simple test to convert PDF to Markdown
use transmutation::{Converter, ConversionOptions, OutputFormat};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Converting PDF to Markdown...");
    
    let pdf_path = PathBuf::from("data/1706.03762v7.pdf");
    let output_path = PathBuf::from("data/test_output.md");
    
    // Create converter
    let converter = Converter::new()?;
    
    // Configure options
    let options = ConversionOptions {
        split_pages: false,
        optimize_for_llm: true,
        extract_tables: false,
        normalize_whitespace: true,
        ..Default::default()
    };
    
    // Convert
    let start = std::time::Instant::now();
    let result = converter
        .convert(&pdf_path)
        .to(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .with_options(options)
        .execute()
        .await?;
    
    let duration = start.elapsed();
    
    // Save output
    result.save(&output_path).await?;
    
    println!("✅ Conversion completed!");
    println!("⏱️  Duration: {:?}", duration);
    println!("📄 Pages: {}", result.statistics.pages_processed);
    println!("📦 Input size: {:.2} MB", result.statistics.input_size_bytes as f64 / 1_000_000.0);
    println!("📦 Output size: {:.2} MB", result.statistics.output_size_bytes as f64 / 1_000_000.0);
    println!("⚡ Speed: {:.2} pages/sec", result.statistics.pages_processed as f64 / duration.as_secs_f64());
    println!("💾 Saved to: {}", output_path.display());
    
    Ok(())
}

