// Debug: See raw PDF extraction
use pdf_extract::extract_text;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting raw text from PDF...");
    
    let raw_text = extract_text("data/1706.03762v7.pdf")?;
    
    // Save raw text
    fs::write("data/raw_extraction.txt", &raw_text)?;
    
    // Show first 100 lines
    let lines: Vec<&str> = raw_text.lines().take(100).collect();
    
    for (i, line) in lines.iter().enumerate() {
        if i > 60 {  // Show lines around author section
            break;
        }
        if i > 0 {  // Skip first few lines
            println!("{:3}: [{}]", i+1, line);
        }
    }
    
    println!("\n💾 Full raw text saved to: data/raw_extraction.txt");
    
    Ok(())
}

