/// Parser for docling-parse JSON output
use serde_json::Value;
use crate::error::Result;
use std::collections::HashMap;

/// Parse docling-parse JSON and convert to Markdown
pub fn parse_docling_json_to_markdown(json_str: &str) -> Result<String> {
    eprintln!("[JSON Parser] Starting parse, input size: {} bytes", json_str.len());
    
    let json: Value = serde_json::from_str(json_str).map_err(|e| {
        eprintln!("[JSON Parser] ERROR parsing JSON: {}", e);
        e
    })?;
    
    eprintln!("[JSON Parser] JSON parsed successfully");
    
    let mut markdown = String::new();
    
    // Extract table of contents for heading detection
    let toc = extract_table_of_contents(&json);
    let heading_titles: HashMap<String, usize> = toc.iter()
        .map(|(title, level)| (title.to_lowercase(), *level))
        .collect();
    
    // Process each page
    if let Some(pages) = json["pages"].as_array() {
        for page in pages {
            if let Some(page_num) = page["page_number"].as_u64() {
                eprintln!("[JSON Parser] Processing page {}", page_num);
            }
            
            // Extract text from lines
            if let Some(lines) = page["original"]["lines"].as_array() {
                let mut prev_line = String::new();
                
                for line in lines {
                    if let Some(line_text) = extract_line_text(line, &page["original"]) {
                        let text = line_text.trim().to_string();
                        if text.is_empty() {
                            continue;
                        }
                        
                        // Check if it's a heading
                        let text_lower = text.to_lowercase();
                        if let Some(&level) = heading_titles.get(&text_lower) {
                            // Add heading
                            let heading_prefix = "#".repeat(level + 1);
                            markdown.push_str(&format!("\n{} {}\n\n", heading_prefix, text));
                        } else {
                            // Check if should join with previous line
                            if should_join_line(&prev_line, &text) {
                                // Remove last newline and add space
                                if markdown.ends_with('\n') {
                                    markdown.pop();
                                }
                                if markdown.ends_with('\n') {
                                    markdown.pop();
                                }
                                markdown.push(' ');
                                markdown.push_str(&text);
                                markdown.push_str("\n\n");
                            } else {
                                // New paragraph
                                markdown.push_str(&text);
                                markdown.push_str("\n\n");
                            }
                        }
                        
                        prev_line = text;
                    }
                }
            }
        }
    }
    
    // Clean up extra newlines
    while markdown.contains("\n\n\n") {
        markdown = markdown.replace("\n\n\n", "\n\n");
    }
    
    Ok(markdown.trim().to_string())
}

/// Extract table of contents
fn extract_table_of_contents(json: &Value) -> Vec<(String, usize)> {
    let mut toc = Vec::new();
    
    if let Some(annotations) = json["annotations"].as_object() {
        if let Some(toc_array) = annotations["table_of_contents"].as_array() {
            extract_toc_recursive(toc_array, &mut toc);
        }
    }
    
    toc
}

/// Recursively extract TOC entries
fn extract_toc_recursive(entries: &[Value], result: &mut Vec<(String, usize)>) {
    for entry in entries {
        if let (Some(title), Some(level)) = (
            entry["title"].as_str(),
            entry["level"].as_u64()
        ) {
            result.push((title.to_string(), level as usize));
        }
        
        // Process children
        if let Some(children) = entry["children"].as_array() {
            extract_toc_recursive(children, result);
        }
    }
}

/// Extract text from a line by reconstructing from cells
fn extract_line_text(line: &Value, original: &Value) -> Option<String> {
    // Lines contain indices that point to cells
    if let Some(indices) = line["i"].as_array() {
        if indices.is_empty() {
            return None;
        }
        
        // Get cell data
        if let Some(cells_obj) = original["cells"].as_object() {
            if let Some(cell_data) = cells_obj["data"].as_array() {
                let mut text = String::new();
                
                // Extract characters from the cell range
                if let (Some(start), Some(end)) = (
                    indices[0].as_u64(),
                    indices.get(1).and_then(|v| v.as_u64())
                ) {
                    for idx in start as usize..end as usize {
                        if let Some(cell) = cell_data.get(idx) {
                            if let Some(cell_array) = cell.as_array() {
                                // Character is at index 12 in the cell array
                                if let Some(ch) = cell_array.get(12).and_then(|v| v.as_str()) {
                                    text.push_str(ch);
                                }
                            }
                        }
                    }
                }
                
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }
    }
    
    None
}

/// Determine if a line should be joined with the previous one
fn should_join_line(prev: &str, current: &str) -> bool {
    if prev.is_empty() {
        return false;
    }
    
    // Check if previous line ends with:
    // - lowercase letter (likely continuation)
    // - comma, semicolon (mid-sentence)
    // - hyphen (word break)
    let last_char = prev.chars().last();
    
    match last_char {
        Some(c) if c.is_lowercase() => true,
        Some(',') | Some(';') | Some('-') => true,
        Some('.') | Some('!') | Some('?') => false, // Sentence end
        _ => {
            // Check if current starts with lowercase (likely continuation)
            current.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_should_join_line() {
        assert!(should_join_line("This is a sentence that continues", "on the next line."));
        assert!(should_join_line("First part,", "second part."));
        assert!(!should_join_line("First sentence.", "Second sentence."));
        assert!(!should_join_line("Question?", "Next question?"));
    }
    
    #[test]
    fn test_extract_toc() {
        let json: Value = serde_json::from_str(r#"{
            "annotations": {
                "table_of_contents": [
                    {"title": "Introduction", "level": 0},
                    {"title": "Methods", "level": 0, "children": [
                        {"title": "Experiment 1", "level": 1}
                    ]}
                ]
            }
        }"#).unwrap();
        
        let toc = extract_table_of_contents(&json);
        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0], ("Introduction".to_string(), 0));
        assert_eq!(toc[1], ("Methods".to_string(), 0));
        assert_eq!(toc[2], ("Experiment 1".to_string(), 1));
    }
}

