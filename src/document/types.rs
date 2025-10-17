use std::collections::HashMap;

/// Document types inspired by docling-core
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for Formatting {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
