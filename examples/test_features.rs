// Test split_pages and JSON export features
use transmutation::{Converter, ConversionOptions, OutputFormat};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf_path = PathBuf::from("data/1706.03762v7.pdf");
    let converter = Converter::new()?;
    
    println!("=== Test 1: Split Pages ===");
    println!("Converting PDF with split_pages=true...\n");
    
    let options = ConversionOptions {
        split_pages: true,
        optimize_for_llm: true,
        ..Default::default()
    };
    
    let result = converter
        .convert(&pdf_path)
        .to(OutputFormat::Markdown {
            split_pages: true,
            optimize_for_llm: true,
        })
        .with_options(options)
        .execute()
        .await?;
    
    println!("✅ Split pages conversion completed!");
    println!("📄 Total outputs: {}", result.content.len());
    for (i, output) in result.content.iter().enumerate() {
        println!("  Page {}: {} bytes", i+1, output.metadata.size_bytes);
    }
    
    // Save split pages (will create a directory)
    result.save(&PathBuf::from("data/pdf_pages.md")).await?;
    println!("💾 Saved {} pages to data/pdf_pages/ directory\n", result.content.len());
    
    // Test 2: JSON export
    println!("=== Test 2: JSON Export ===");
    println!("Converting PDF to JSON...\n");
    
    let options = ConversionOptions {
        include_metadata: true,
        optimize_for_llm: true,
        ..Default::default()
    };
    
    let result = converter
        .convert(&pdf_path)
        .to(OutputFormat::Json {
            structured: true,
            include_metadata: true,
        })
        .with_options(options)
        .execute()
        .await?;
    
    // Save JSON
    result.save(&PathBuf::from("data/document_metadata.json")).await?;
    
    println!("✅ JSON export completed!");
    println!("📦 Output size: {} bytes", result.statistics.output_size_bytes);
    println!("💾 Saved to: data/document_metadata.json\n");
    
    // Show metadata
    if let Some(title) = &result.metadata.title {
        println!("📚 Title: {}", title);
    }
    if let Some(author) = &result.metadata.author {
        println!("👤 Author: {}", author);
    }
    println!("📄 Pages: {}", result.metadata.page_count);
    
    println!("\n🎉 All features tested successfully!");
    
    Ok(())
}

