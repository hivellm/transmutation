use std::collections::HashMap;

/// Document hierarchy builder - creates section trees and relationships
///
/// Based on docling-core document structure
use crate::document::types::{DocItem, DoclingDocument, ListItemData, SectionHeaderItem};
use crate::error::{Result, TransmutationError};

/// Hierarchy builder for document structure
pub struct HierarchyBuilder {
    enable_section_tree: bool,
    enable_list_grouping: bool,
    enable_caption_pairing: bool,
}

impl HierarchyBuilder {
    pub fn new() -> Self {
        Self {
            enable_section_tree: true,
            enable_list_grouping: true,
            enable_caption_pairing: true,
        }
    }

    /// Build complete document with hierarchy from items
    pub fn build(&self, filename: String, mut items: Vec<DocItem>) -> Result<DoclingDocument> {
        // Build section tree if enabled
        if self.enable_section_tree {
            items = self.build_section_tree(items)?;
        }

        // Group consecutive list items if enabled
        if self.enable_list_grouping {
            items = self.group_list_items(items)?;
        }

        // Pair captions with figures/tables if enabled
        if self.enable_caption_pairing {
            items = self.pair_captions(items)?;
        }

        let mut doc = DoclingDocument::new(filename);

        for item in items {
            doc.add_item(item);
        }

        Ok(doc)
    }

    /// Build section tree - ensure proper nesting of section headers
    ///
    /// This doesn't restructure the flat list, but validates levels are consistent
    fn build_section_tree(&self, items: Vec<DocItem>) -> Result<Vec<DocItem>> {
        let mut result = Vec::new();
        let mut current_level = 0;

        for item in items {
            match &item {
                DocItem::SectionHeader(header) => {
                    // Ensure level progression is reasonable (no jumps > 1)
                    let adjusted_level = if header.level > current_level + 1 {
                        current_level + 1
                    } else {
                        header.level
                    };

                    current_level = adjusted_level;

                    result.push(DocItem::SectionHeader(SectionHeaderItem {
                        level: adjusted_level,
                        ..header.clone()
                    }));
                }
                _ => {
                    result.push(item);
                }
            }
        }

        Ok(result)
    }

    /// Group consecutive list items (future: could create ListGroup items)
    ///
    /// For now, just detects and validates list nesting
    fn group_list_items(&self, items: Vec<DocItem>) -> Result<Vec<DocItem>> {
        let mut result = Vec::new();
        let mut current_list: Vec<ListItemData> = Vec::new();
        let mut prev_level = 0;

        for item in items {
            match item {
                DocItem::ListItem(ref list_item) => {
                    // Adjust level based on previous items
                    let adjusted_level = if list_item.level > prev_level + 1 {
                        prev_level + 1
                    } else {
                        list_item.level
                    };

                    prev_level = adjusted_level;

                    current_list.push(ListItemData {
                        level: adjusted_level,
                        ..list_item.clone()
                    });
                }
                _ => {
                    // Flush accumulated list items
                    for list_item in current_list.drain(..) {
                        result.push(DocItem::ListItem(list_item));
                    }
                    prev_level = 0;
                    result.push(item);
                }
            }
        }

        // Flush remaining list items
        for list_item in current_list {
            result.push(DocItem::ListItem(list_item));
        }

        Ok(result)
    }

    /// Pair captions with their corresponding figures/tables
    ///
    /// Looks for caption items immediately before/after figure/table items
    fn pair_captions(&self, items: Vec<DocItem>) -> Result<Vec<DocItem>> {
        if items.len() < 2 {
            return Ok(items);
        }

        let mut result = Vec::new();
        let mut i = 0;

        while i < items.len() {
            let item = &items[i];

            match item {
                &DocItem::Table(ref table) if table.caption.is_none() => {
                    // Check if next item is caption (caption after table)
                    if i + 1 < items.len() {
                        if let DocItem::Paragraph(ref text_item) = items[i + 1] {
                            if Self::is_likely_caption(&text_item.text) {
                                // Merge caption into table
                                let mut new_table = table.clone();
                                new_table.caption = Some(text_item.text.clone());
                                result.push(DocItem::Table(new_table));
                                i += 2; // Skip caption
                                continue;
                            }
                        }
                    }

                    // Check if previous item was caption (caption before table)
                    if !result.is_empty() {
                        // Clone text before mutating result
                        let caption_text =
                            if let Some(DocItem::Paragraph(text_item)) = result.last() {
                                if Self::is_likely_caption(&text_item.text) {
                                    Some(text_item.text.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                        if let Some(caption) = caption_text {
                            result.pop();
                            let mut new_table = table.clone();
                            new_table.caption = Some(caption);
                            result.push(DocItem::Table(new_table));
                            i += 1;
                            continue;
                        }
                    }

                    result.push(item.clone());
                    i += 1;
                }
                &DocItem::Picture(ref picture) if picture.caption.is_none() => {
                    // Similar logic for pictures
                    if i + 1 < items.len() {
                        if let DocItem::Paragraph(ref text_item) = items[i + 1] {
                            if Self::is_likely_caption(&text_item.text) {
                                let mut new_picture = picture.clone();
                                new_picture.caption = Some(text_item.text.clone());
                                result.push(DocItem::Picture(new_picture));
                                i += 2;
                                continue;
                            }
                        }
                    }

                    if !result.is_empty() {
                        // Clone text before mutating result
                        let caption_text =
                            if let Some(DocItem::Paragraph(text_item)) = result.last() {
                                if Self::is_likely_caption(&text_item.text) {
                                    Some(text_item.text.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                        if let Some(caption) = caption_text {
                            result.pop();
                            let mut new_picture = picture.clone();
                            new_picture.caption = Some(caption);
                            result.push(DocItem::Picture(new_picture));
                            i += 1;
                            continue;
                        }
                    }

                    result.push(item.clone());
                    i += 1;
                }
                _ => {
                    result.push(item.clone());
                    i += 1;
                }
            }
        }

        Ok(result)
    }

    /// Detect if text is likely a caption
    fn is_likely_caption(text: &str) -> bool {
        let lower = text.to_lowercase();

        // Common caption patterns
        lower.starts_with("figure ")
            || lower.starts_with("fig. ")
            || lower.starts_with("table ")
            || lower.starts_with("image ")
            || lower.starts_with("chart ")
            || lower.starts_with("graph ")
            || (lower.len() < 100 && (lower.contains("figure") || lower.contains("table")))
    }
}

impl Default for HierarchyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Build relationships between document items
///
/// This creates a graph of relationships like:
/// - Section contains paragraphs
/// - Caption references figure
/// - Footnote references paragraph
pub struct RelationshipBuilder {
    relationships: HashMap<String, Vec<String>>,
}

impl RelationshipBuilder {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// Add a relationship between two items
    pub fn add_relationship(&mut self, from: String, to: String, rel_type: &str) {
        let key = format!("{}:{}", from, rel_type);
        self.relationships
            .entry(key)
            .or_insert_with(Vec::new)
            .push(to);
    }

    /// Get relationships for an item
    pub fn get_relationships(&self, item_ref: &str, rel_type: &str) -> Vec<String> {
        let key = format!("{}:{}", item_ref, rel_type);
        self.relationships.get(&key).cloned().unwrap_or_default()
    }

    /// Build relationships from document structure
    pub fn build_from_document(&mut self, doc: &DoclingDocument) -> Result<()> {
        let mut current_section: Option<String> = None;
        let mut _current_figure: Option<String> = None;

        for (idx, item) in doc.items.iter().enumerate() {
            let item_ref = format!("item_{}", idx);

            match item {
                DocItem::SectionHeader(_) => {
                    current_section = Some(item_ref.clone());
                }
                DocItem::Paragraph(_) | DocItem::ListItem(_) => {
                    // Link to current section
                    if let Some(ref section) = current_section {
                        self.add_relationship(section.clone(), item_ref.clone(), "contains");
                    }
                }
                DocItem::Table(_) | DocItem::Picture(_) => {
                    _current_figure = Some(item_ref.clone());

                    // Link to current section
                    if let Some(ref section) = current_section {
                        self.add_relationship(section.clone(), item_ref.clone(), "contains");
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Default for RelationshipBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::types::{DocItemLabel, Formatting, TextItem};

    #[test]
    fn test_is_likely_caption() {
        assert!(HierarchyBuilder::is_likely_caption(
            "Figure 1: This is a test"
        ));
        assert!(HierarchyBuilder::is_likely_caption("Table 2: Results"));
        assert!(HierarchyBuilder::is_likely_caption("Fig. 3: Sample data"));
        assert!(!HierarchyBuilder::is_likely_caption(
            "This is a regular paragraph with no caption markers."
        ));
    }

    #[test]
    fn test_section_tree_validation() {
        let builder = HierarchyBuilder::new();

        let items = vec![
            DocItem::Title(TextItem {
                text: "Title".to_string(),
                formatting: None,
                label: DocItemLabel::Title,
            }),
            DocItem::SectionHeader(SectionHeaderItem {
                text: "Section 1".to_string(),
                level: 1,
                formatting: None,
            }),
            DocItem::SectionHeader(SectionHeaderItem {
                text: "Section 1.1".to_string(),
                level: 5, // Invalid jump - should be corrected to 2
                formatting: None,
            }),
        ];

        let result = builder.build_section_tree(items).unwrap();

        if let DocItem::SectionHeader(header) = &result[2] {
            assert_eq!(header.level, 2); // Should be corrected from 5 to 2
        } else {
            panic!("Expected SectionHeader");
        }
    }

    #[test]
    fn test_caption_pairing() {
        let builder = HierarchyBuilder::new();

        let items = vec![
            DocItem::Paragraph(TextItem {
                text: "Figure 1: A beautiful chart".to_string(),
                formatting: None,
                label: DocItemLabel::Caption,
            }),
            DocItem::Picture(crate::document::types::PictureItem {
                caption: None,
                placeholder: "image".to_string(),
            }),
        ];

        let result = builder.pair_captions(items).unwrap();

        assert_eq!(result.len(), 1); // Caption should be merged

        if let DocItem::Picture(picture) = &result[0] {
            assert!(picture.caption.is_some());
            assert_eq!(
                picture.caption.as_ref().unwrap(),
                "Figure 1: A beautiful chart"
            );
        } else {
            panic!("Expected Picture");
        }
    }
}
