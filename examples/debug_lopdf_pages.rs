// Debug: See what lopdf extracts from each page
use transmutation::engines::pdf_parser::PdfParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting pages with lopdf...\n");
    
    let parser = PdfParser::load("data/1706.03762v7.pdf")?;
    let pages = parser.extract_all_pages()?;
    
    println!("📄 Total pages: {}\n", pages.len());
    
    for (i, page) in pages.iter().take(3).enumerate() {
        println!("=== Page {} ===", i + 1);
        println!("Text length: {} chars", page.text.len());
        println!("Lines count: {}", page.text.lines().count());
        println!("First 300 chars:");
        println!("{}", &page.text.chars().take(300).collect::<String>());
        println!("\nFirst 10 lines:");
        for (j, line) in page.text.lines().take(10).enumerate() {
            println!("  {}: [{}]", j+1, line);
        }
        println!();
    }
    
    Ok(())
}

