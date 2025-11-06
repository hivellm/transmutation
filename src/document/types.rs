//! Document types inspired by docling-core

#![allow(missing_docs)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoclingDocument {
    pub name: String,
    pub items: Vec<DocItem>,
    #[serde(skip)]
    pub items_by_ref: HashMap<String, usize>,
}

impl DoclingDocument {
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
            items_by_ref: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: DocItem) -> String {
        let item_ref = format!("item_{}", self.items.len());
        self.items_by_ref.insert(item_ref.clone(), self.items.len());
        self.items.push(item);
        item_ref
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DocItem {
    Title(TextItem),
    SectionHeader(SectionHeaderItem),
    Paragraph(TextItem),
    ListItem(ListItemData),
    Table(TableItem),
    Picture(PictureItem),
    Code(CodeItem),
    Formula(FormulaItem),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextItem {
    pub text: String,
    pub formatting: Option<Formatting>,
    pub label: DocItemLabel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionHeaderItem {
    pub text: String,
    pub level: usize,
    pub formatting: Option<Formatting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemData {
    pub text: String,
    pub marker: String,
    pub enumerated: bool,
    pub level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableItem {
    pub data: TableData,
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub num_rows: usize,
    pub num_cols: usize,
    pub grid: Vec<Vec<TableCell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub text: String,
    pub row_span: usize,
    pub col_span: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PictureItem {
    pub caption: Option<String>,
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeItem {
    pub text: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaItem {
    pub text: String,
    pub is_inline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Formatting {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocItemLabel {
    Title,
    SectionHeader,
    Paragraph,
    Text,
    Caption,
    Footnote,
    PageHeader,
    PageFooter,
    ListItem,
    Code,
    Formula,
    Table,
    Picture,
    Figure, // Alias for Picture (docling compatibility)
    CheckboxSelected,
    CheckboxUnselected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docling_document_creation() {
        let doc = DoclingDocument::new("test.pdf".to_string());
        assert_eq!(doc.name, "test.pdf");
        assert!(doc.items.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut doc = DoclingDocument::new("test".to_string());
        let item = DocItem::Paragraph(TextItem {
            text: "Test".to_string(),
            formatting: None,
            label: DocItemLabel::Paragraph,
        });
        let item_ref = doc.add_item(item);
        assert_eq!(doc.items.len(), 1);
        assert_eq!(item_ref, "item_0");
    }

    #[test]
    fn test_formatting_default() {
        let formatting = Formatting::default();
        assert!(!formatting.bold);
        assert!(!formatting.italic);
        assert!(!formatting.underline);
    }

    #[test]
    fn test_table_cell_creation() {
        let cell = TableCell {
            text: "Cell".to_string(),
            row_span: 1,
            col_span: 1,
        };
        assert_eq!(cell.text, "Cell");
        assert_eq!(cell.row_span, 1);
    }

    #[test]
    fn test_section_header_item() {
        let header = SectionHeaderItem {
            text: "Section 1".to_string(),
            level: 1,
            formatting: None,
        };
        assert_eq!(header.text, "Section 1");
        assert_eq!(header.level, 1);
    }

    #[test]
    fn test_list_item_data() {
        let list_item = ListItemData {
            text: "Item 1".to_string(),
            marker: "-".to_string(),
            enumerated: false,
            level: 0,
        };
        assert_eq!(list_item.text, "Item 1");
        assert!(!list_item.enumerated);
    }

    #[test]
    fn test_table_data_creation() {
        let table = TableData {
            num_rows: 2,
            num_cols: 3,
            grid: vec![],
        };
        assert_eq!(table.num_rows, 2);
        assert_eq!(table.num_cols, 3);
    }
}
