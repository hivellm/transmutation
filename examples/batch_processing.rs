//! Batch processing multiple documents
//!
//! This example demonstrates how to convert multiple PDF files in parallel.

#![allow(unused_variables, clippy::uninlined_format_args)]

use std::path::PathBuf;

use futures::future::join_all;
use transmutation::{ConversionOptions, Converter, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::new()?;

    // List of files to process
    let input_files = vec![
        "document1.pdf",
        "document2.pdf",
        "document3.pdf",
        // Add more files as needed
    ];

    println!(
        "Starting batch conversion of {} files...\n",
        input_files.len()
    );

    // Create conversion tasks
    let tasks: Vec<_> = input_files
        .into_iter()
        .map(|file| {
            let converter = Converter::new().unwrap();
            let file = file.to_string();

            async move {
                println!("🔄 Processing: {}", file);

                let output_file = PathBuf::from(&file).with_extension("md");

                let result = converter
                    .convert(&file)
                    .to(OutputFormat::Markdown {
                        split_pages: false,
                        optimize_for_llm: true,
                    })
                    .with_options(ConversionOptions::default())
                    .execute()
                    .await;

                match result {
                    Ok(res) => {
                        res.save(&output_file).await?;
                        println!("✅ Completed: {} -> {:?}", file, output_file);
                        Ok::<_, Box<dyn std::error::Error>>(res.statistics)
                    }
                    Err(e) => {
                        eprintln!("❌ Failed: {} - {:?}", file, e);
                        Err(e.into())
                    }
                }
            }
        })
        .collect();

    // Execute all conversions in parallel
    let results = join_all(tasks).await;

    // Summarize results
    let mut successful = 0;
    let mut failed = 0;
    let mut total_pages = 0;
    let mut total_duration = std::time::Duration::ZERO;

    for result in results {
        match result {
            Ok(stats) => {
                successful += 1;
                total_pages += stats.pages_processed;
                total_duration += stats.duration;
            }
            Err(_) => {
                failed += 1;
            }
        }
    }

    // Display summary
    println!("\n📊 Batch Processing Summary:");
    println!("  ✅ Successful: {}", successful);
    println!("  ❌ Failed: {}", failed);
    println!("  📄 Total pages: {}", total_pages);
    println!("  ⏱️  Total time: {:?}", total_duration);

    if successful > 0 {
        println!(
            "  ⚡ Average speed: {:.2} pages/second",
            total_pages as f64 / total_duration.as_secs_f64()
        );
    }

    Ok(())
}
