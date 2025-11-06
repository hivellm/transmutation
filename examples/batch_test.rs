//! Batch processing example
//!
//! Demonstrates parallel conversion of multiple documents

#![allow(unused_variables, unused_imports, clippy::uninlined_format_args)]

use transmutation::{BatchProcessor, ConversionOptions, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Batch Processing Example\n");

    // Create batch processor
    let batch = BatchProcessor::new()
        .add_files(&[
            "data/office_samples/test.txt",
            "data/office_samples/test2.csv",
            "data/office_samples/test.html",
            "data/office_samples/test.xml",
        ])
        .output_format(OutputFormat::Markdown {
            split_pages: false,
            optimize_for_llm: true,
        })
        .parallel(4);

    // Execute batch conversion
    let result = batch.execute().await?;

    // Print results
    println!("\n📊 Batch Results:");
    println!("   Total files: {}", result.total_files);
    println!("   Success: {}", result.successes.len());
    println!("   Failed: {}", result.failures.len());
    println!("   Success rate: {:.1}%", result.success_rate());
    println!("   Total pages: {}", result.total_pages());
    println!("   Total time: {:.2}s", result.total_time.as_secs_f64());
    println!("   Speed: {:.1} pages/sec", result.pages_per_second());

    // Save all outputs
    result.save_all("data/office_samples/batch_output").await?;
    println!("\n✅ All outputs saved to data/office_samples/batch_output/");

    Ok(())
}
