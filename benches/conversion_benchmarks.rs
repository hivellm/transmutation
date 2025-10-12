//! Performance benchmarks for Transmutation
//!
//! These benchmarks measure the performance of core conversion operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use transmutation::engines::pdf_parser::PdfParser;
use transmutation::engines::table_detector::TableDetector;
use transmutation::output::{Chunker, ChunkStrategy, MarkdownGenerator};
use transmutation::types::ConversionOptions;

fn benchmark_markdown_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("markdown_generation");

    // Sample text of different sizes
    let small_text = "Hello world! ".repeat(10);
    let medium_text = "This is a test paragraph. ".repeat(100);
    let large_text = "Lorem ipsum dolor sit amet. ".repeat(1000);

    group.bench_with_input(
        BenchmarkId::new("small", small_text.len()),
        &small_text,
        |b, text| {
            b.iter(|| {
                let opts = ConversionOptions::default();
                MarkdownGenerator::from_text(black_box(text), opts)
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("medium", medium_text.len()),
        &medium_text,
        |b, text| {
            b.iter(|| {
                let opts = ConversionOptions::default();
                MarkdownGenerator::from_text(black_box(text), opts)
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("large", large_text.len()),
        &large_text,
        |b, text| {
            b.iter(|| {
                let opts = ConversionOptions::default();
                MarkdownGenerator::from_text(black_box(text), opts)
            })
        },
    );

    group.finish();
}

fn benchmark_text_chunking(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_chunking");

    let text = "This is a test sentence. ".repeat(1000);
    group.throughput(Throughput::Bytes(text.len() as u64));

    group.bench_function("token_based", |b| {
        let chunker = Chunker::new(ChunkStrategy::TokenBased, 512, 50);
        b.iter(|| chunker.chunk(black_box(&text)))
    });

    group.bench_function("semantic", |b| {
        let chunker = Chunker::new(ChunkStrategy::Semantic, 512, 0);
        b.iter(|| chunker.chunk(black_box(&text)))
    });

    group.bench_function("sliding_window", |b| {
        let chunker = Chunker::new(ChunkStrategy::SlidingWindow, 512, 100);
        b.iter(|| chunker.chunk(black_box(&text)))
    });

    group.finish();
}

fn benchmark_table_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_detection");

    // Pipe-delimited table
    let pipe_table = r#"
| Name | Age | City |
| --- | --- | --- |
| Alice | 30 | NYC |
| Bob | 25 | LA |
| Charlie | 35 | SF |
"#;

    // Tab-separated table
    let tab_table = "Name\tAge\tCity\nAlice\t30\tNYC\nBob\t25\tLA\nCharlie\t35\tSF";

    // Whitespace-aligned table
    let aligned_table = r#"
Name      Age    City
Alice     30     NYC
Bob       25     LA
Charlie   35     SF
"#;

    let detector = TableDetector::new();

    group.bench_function("pipe_delimited", |b| {
        b.iter(|| detector.detect_tables(black_box(pipe_table)))
    });

    group.bench_function("tab_separated", |b| {
        b.iter(|| detector.detect_tables(black_box(tab_table)))
    });

    group.bench_function("whitespace_aligned", |b| {
        b.iter(|| detector.detect_tables(black_box(aligned_table)))
    });

    group.finish();
}

fn benchmark_text_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_optimization");

    let text_with_issues = "Hello    world!\n\n\n\nTest    text   with    issues.   ";

    group.bench_function("whitespace_normalization", |b| {
        b.iter(|| {
            let mut opts = ConversionOptions::default();
            opts.normalize_whitespace = true;
            opts.optimize_for_llm = true;
            MarkdownGenerator::from_text(black_box(text_with_issues), opts)
        })
    });

    group.finish();
}

fn benchmark_end_to_end_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.sample_size(10); // Fewer samples for memory-intensive tests

    // Simulate varying document sizes
    for size in [100, 1000, 10000].iter() {
        let pages: Vec<(usize, String)> = (0..*size)
            .map(|i| (i, format!("Page {} content. ", i).repeat(50)))
            .collect();

        group.bench_with_input(BenchmarkId::new("pages", size), &pages, |b, pages| {
            b.iter(|| {
                let opts = ConversionOptions::default();
                MarkdownGenerator::from_pages(black_box(pages), opts)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_markdown_generation,
    benchmark_text_chunking,
    benchmark_table_detection,
    benchmark_text_optimization,
    benchmark_end_to_end_memory
);
criterion_main!(benches);
