//! Test FFI conversion with docling-parse
//!
//! This example tests the full FFI pipeline with ML models

use transmutation::converters::pdf::PdfConverter;
use transmutation::converters::DocumentConverter;
use transmutation::types::{ConversionOptions, OutputFormat};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  🚀 Testing Docling FFI Pipeline      ║");
    println!("╚════════════════════════════════════════╝\n");

    // Test PDF path
    let pdf_path = Path::new("data/1706.03762v7.pdf");
    
    if !pdf_path.exists() {
        eprintln!("❌ PDF not found: {}", pdf_path.display());
        eprintln!("📁 Available PDFs:");
        for entry in std::fs::read_dir("data")? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("pdf") {
                println!("   - {}", entry.path().display());
            }
        }
        return Ok(());
    }
    
    println!("📄 Input PDF: {}", pdf_path.display());
    println!("🔧 Features: docling-ffi enabled");
    println!("🧠 Pipeline: Full ML + Post-processing\n");

    // Create PDF converter
    let converter = PdfConverter::new();
    
    let mut options = ConversionOptions::default();
    options.use_ffi = true;  // Enable FFI path
    
    let output_format = OutputFormat::Markdown {
        split_pages: false,
        optimize_for_llm: true,
    };

    println!("⏳ Converting PDF...\n");
    
    // Convert
    let result = converter.convert(pdf_path, output_format, options).await?;

    println!("\n✅ Conversion complete!");
    println!("📊 Pages processed: {}", result.statistics.pages_processed);
    println!("📊 Output chunks: {}", result.content.len());
    
    // Combine all outputs
    let mut full_output = String::new();
    for output in &result.content {
        full_output.push_str(&String::from_utf8_lossy(&output.data));
        full_output.push('\n');
    }
    
    println!("📏 Total size: {} bytes", full_output.len());
    println!("📏 Lines: {}", full_output.lines().count());
    
    // Save output
    let output_path = "data/output_ffi_test.md";
    std::fs::write(output_path, &full_output)?;
    
    println!("\n💾 Saved to: {}", output_path);
    
    // Show first 50 lines
    println!("\n╔════════════════════════════════════════╗");
    println!("║  📝 First 50 lines of output:         ║");
    println!("╚════════════════════════════════════════╝\n");
    
    for (i, line) in full_output.lines().take(50).enumerate() {
        println!("{:3} | {}", i + 1, line);
    }
    
    if full_output.lines().count() > 50 {
        println!("\n... ({} more lines)", full_output.lines().count() - 50);
    }

    Ok(())
}

