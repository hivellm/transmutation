/// Pipeline performance benchmarks
/// 
/// Measures:
/// - Conversion time per page
/// - Memory usage
/// - Output quality metrics
/// 
/// Run with: cargo bench --features docling-ffi

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use transmutation::converters::pdf::PdfConverter;
use transmutation::converters::DocumentConverter;
use transmutation::types::{ConversionOptions, OutputFormat};
use std::path::Path;
use std::time::Duration;

fn benchmark_pdf_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("pdf_conversion");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);
    
    // Test PDF (adjust path as needed)
    let test_pdf = "data/1706.03762v7.pdf";
    
    if !Path::new(test_pdf).exists() {
        eprintln!("⚠️  Test PDF not found: {}", test_pdf);
        eprintln!("   Skipping benchmarks");
        return;
    }
    
    let converter = PdfConverter::new();
    let output_format = OutputFormat::Markdown {
        split_pages: false,
        optimize_for_llm: true,
    };
    
    // Benchmark with FFI (if available)
    #[cfg(feature = "docling-ffi")]
    {
        group.bench_with_input(
            BenchmarkId::new("with_ffi", "15pages"),
            &test_pdf,
            |b, path| {
                b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
                    let mut options = ConversionOptions::default();
                    options.use_ffi = true;
                    
                    let result = converter
                        .convert(Path::new(black_box(path)), output_format.clone(), options)
                        .await
                        .expect("Conversion failed");
                    
                    black_box(result)
                });
            },
        );
    }
    
    // Benchmark without FFI (precision mode)
    group.bench_with_input(
        BenchmarkId::new("precision", "15pages"),
        &test_pdf,
        |b, path| {
            b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
                let mut options = ConversionOptions::default();
                options.use_ffi = false;
                
                let result = converter
                    .convert(Path::new(black_box(path)), output_format.clone(), options)
                    .await
                    .expect("Conversion failed");
                
                black_box(result)
            });
        },
    );
    
    group.finish();
}

fn benchmark_text_sanitization(c: &mut Criterion) {
    use transmutation::document::text_utils::TextSanitizer;
    
    let mut group = c.benchmark_group("text_sanitization");
    
    let sample_texts = vec![
        ("short", "This is a simple text with — dashes and "quotes"."),
        ("medium", "The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism."),
        ("long", "The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely. Experiments on two machine translation tasks show these models to be superior in quality while being more parallelizable and requiring significantly less time to train."),
    ];
    
    let sanitizer = TextSanitizer::new();
    
    for (name, text) in sample_texts {
        group.bench_with_input(
            BenchmarkId::new("sanitize", name),
            &text,
            |b, text| {
                b.iter(|| {
                    sanitizer.sanitize(black_box(text))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_parser(c: &mut Criterion) {
    use transmutation::document::DoclingJsonParser;
    
    let mut group = c.benchmark_group("json_parser");
    
    // Sample JSON (minimal for benchmark)
    let sample_json = r#"{
        "info": {"filename": "test.pdf"},
        "annotations": {"table_of_contents": []},
        "pages": [{
            "original": {
                "cells": {
                    "data": [
                        [10.0, 100.0, 50.0, 110.0, 40.0, 10.0, null, null, null, null, null, null, "Sample text"]
                    ]
                }
            }
        }]
    }"#;
    
    group.bench_function("parse_json", |b| {
        b.iter(|| {
            DoclingJsonParser::parse(black_box(sample_json))
                .expect("Parse failed")
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_pdf_conversion,
    benchmark_text_sanitization,
    benchmark_parser
);

criterion_main!(benches);

