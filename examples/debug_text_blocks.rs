use transmutation::engines::pdf_parser::PdfParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PdfParser::load("data/1706.03762v7.pdf")?;
    let pages = parser.extract_all_pages()?;
    
    println!("Total pages: {}", pages.len());
    
    for (i, page) in pages.iter().take(2).enumerate() {
        println!("\n=== Page {} ===", i);
        println!("Text blocks: {}", page.text_blocks.len());
        println!("Width: {}, Height: {}", page.width, page.height);
        println!("Text preview: {:?}", page.text.chars().take(200).collect::<String>());
        
        for (j, block) in page.text_blocks.iter().take(10).enumerate() {
            println!("  Block {}: x={:.1}, y={:.1}, font={:.1}, text={:?}", 
                j, block.x, block.y, block.font_size, 
                block.text.chars().take(50).collect::<String>());
        }
    }
    
    Ok(())
}

