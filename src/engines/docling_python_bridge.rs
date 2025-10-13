/// Bridge to Python docling for layout analysis
/// 
/// This module calls Python docling to get proper layout analysis
/// until we have ONNX models working.

use crate::error::{Result, TransmutationError};
use crate::document::types_extended::{Cluster, BoundingBox, CoordOrigin, TextCell};
use crate::document::types::DocItemLabel;
use std::path::Path;
use std::process::Command;
use serde_json::Value;

/// Get layout clusters from Python docling
pub fn get_layout_clusters(pdf_path: &Path) -> Result<Vec<Cluster>> {
    eprintln!("🐍 Calling Python docling for layout analysis...");
    
    // Create a temporary Python script  
    let script = r#"
import sys
import json
from pathlib import Path
from docling.document_converter import DocumentConverter

try:
    # Convert PDF
    converter = DocumentConverter()
    pdf_path = Path(sys.argv[1])
    result = converter.convert(pdf_path)
    
    # Extract layout information
    clusters = []
    cluster_id = 0
    
    # Iterate through document items
    for item in result.document.items:
        # Get bounding box
        if hasattr(item, 'prov') and hasattr(item.prov, 'bbox'):
            bbox = item.prov.bbox
            
            cluster = {{
                "id": cluster_id,
                "label": str(item.label) if hasattr(item, 'label') else "text",
                "bbox": {{
                    "l": bbox.l,
                    "t": bbox.t,
                    "r": bbox.r,
                    "b": bbox.b
                }},
                "text": str(item.text) if hasattr(item, 'text') else "",
                "confidence": 1.0
            }}
            
            clusters.append(cluster)
            cluster_id += 1
    
    # Output JSON
    print(json.dumps(clusters))
    sys.exit(0)
    
except Exception as e:
    print(json.dumps({{"error": str(e)}}), file=sys.stderr)
    sys.exit(1)
"#.to_string();
    
    // Write script to temp file
    let script_path = "/tmp/docling_bridge.py";
    std::fs::write(script_path, script)?;
    
    // Execute Python script
    let output = Command::new("python3")
        .arg(script_path)
        .arg(pdf_path)
        .output()
        .map_err(|e| TransmutationError::EngineError {
            engine: "docling-python-bridge".to_string(),
            message: format!("Failed to execute Python: {}", e),
            source: None,
        })?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TransmutationError::EngineError {
            engine: "docling-python-bridge".to_string(),
            message: format!("Python docling failed: {}", stderr),
            source: None,
        });
    }
    
    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)
        .map_err(|e| TransmutationError::EngineError {
            engine: "docling-python-bridge".to_string(),
            message: format!("Failed to parse JSON: {}", e),
            source: None,
        })?;
    
    // Convert to Rust clusters
    let clusters_array = json.as_array()
        .ok_or_else(|| TransmutationError::EngineError {
            engine: "docling-python-bridge".to_string(),
            message: "Expected array of clusters".to_string(),
            source: None,
        })?;
    
    let mut clusters = Vec::new();
    
    for cluster_json in clusters_array {
        let id = cluster_json["id"].as_u64().unwrap_or(0) as usize;
        let label_str = cluster_json["label"].as_str().unwrap_or("text");
        let label = parse_label(label_str);
        
        let bbox_json = &cluster_json["bbox"];
        let bbox = BoundingBox {
            l: bbox_json["l"].as_f64().unwrap_or(0.0),
            t: bbox_json["t"].as_f64().unwrap_or(0.0),
            r: bbox_json["r"].as_f64().unwrap_or(0.0),
            b: bbox_json["b"].as_f64().unwrap_or(0.0),
            origin: CoordOrigin::TopLeft,
        };
        
        let text = cluster_json["text"].as_str().unwrap_or("").to_string();
        let confidence = cluster_json["confidence"].as_f64().unwrap_or(1.0) as f32;
        
        // Create text cell from cluster
        let cell = TextCell {
            index: id,
            text: text.clone(),
            bbox,
            font_name: None,
            font_size: None,
            confidence,
            from_ocr: false,
        };
        
        clusters.push(Cluster {
            id,
            label,
            bbox,
            cells: vec![cell],
            confidence,
        });
    }
    
    eprintln!("      ✓ Got {} clusters from Python docling", clusters.len());
    
    Ok(clusters)
}

fn parse_label(label_str: &str) -> DocItemLabel {
    match label_str.to_lowercase().as_str() {
        "title" => DocItemLabel::Title,
        "section_header" | "sectionheader" => DocItemLabel::SectionHeader,
        "paragraph" => DocItemLabel::Paragraph,
        "list_item" | "listitem" => DocItemLabel::ListItem,
        "table" => DocItemLabel::Table,
        "picture" | "figure" => DocItemLabel::Picture,
        "code" => DocItemLabel::Code,
        "formula" => DocItemLabel::Formula,
        "caption" => DocItemLabel::Caption,
        "footnote" => DocItemLabel::Footnote,
        _ => DocItemLabel::Text,
    }
}

