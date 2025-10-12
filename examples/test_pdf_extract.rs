//! Test pdf-extract vs lopdf extraction quality

use pdf_extract::extract_text;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data/1706.03762v7.pdf");
    
    println!("🔍 Testing pdf-extract on Attention paper...\n");
    
    let text = extract_text(path)?;
    
    // Check length
    println!("📊 Total characters extracted: {}", text.len());
    
    // Check if Abstract exists
    if text.contains("Abstract") {
        println!("✅ Found 'Abstract'");
        if let Some(pos) = text.find("Abstract") {
            let start = pos.saturating_sub(100);
            let end = (pos + 600).min(text.len());
            println!("\n📄 Context around Abstract:\n{}", &text[start..end]);
        }
    } else {
        println!("❌ 'Abstract' NOT FOUND");
    }
    
    // Check if "dominant" exists (first word of abstract)
    if text.contains("dominant") {
        println!("\n✅ Found 'dominant' (first word of Abstract text)");
    } else {
        println!("\n❌ 'dominant' NOT FOUND");
    }
    
    // Show first 2000 chars
    println!("\n📝 First 2000 characters:\n{}", &text[..text.len().min(2000)]);
    
    Ok(())
}

