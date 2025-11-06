//! HTML converter implementation
//!
//! Converts HTML to Markdown using semantic HTML parsing.
//! Preserves structure, links, images, and formatting.

#![allow(
    clippy::unused_self,
    clippy::uninlined_format_args,
    clippy::unnecessary_wraps,
    clippy::single_char_add_str
)]

use std::path::Path;

use async_trait::async_trait;
use tokio::fs;

use super::traits::{ConverterMetadata, DocumentConverter};
use crate::Result;
use crate::types::{
    ConversionOptions, ConversionOutput, ConversionResult, FileFormat, OutputFormat, OutputMetadata,
};

/// HTML to Markdown converter
#[derive(Debug)]
pub struct HtmlConverter;

impl HtmlConverter {
    /// Create a new HTML converter
    pub fn new() -> Self {
        Self
    }

    /// Convert HTML to Markdown
    fn html_to_markdown(&self, html: &str) -> Result<String> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        let mut markdown = String::new();

        // Extract title
        if let Ok(selector) = Selector::parse("title") {
            if let Some(title) = document.select(&selector).next() {
                markdown.push_str(&format!("# {}\n\n", title.inner_html().trim()));
            }
        }

        // Extract main content (try multiple selectors)
        let content_selectors = vec!["main", "article", "body"];
        let mut content_found = false;

        for sel_str in content_selectors {
            if let Ok(selector) = Selector::parse(sel_str) {
                if let Some(main_content) = document.select(&selector).next() {
                    markdown.push_str(&self.process_element(&main_content));
                    content_found = true;
                    break;
                }
            }
        }

        if !content_found {
            // Fallback: extract all text
            markdown.push_str(&document.root_element().text().collect::<Vec<_>>().join(" "));
        }

        Ok(markdown)
    }

    /// Process HTML element recursively
    fn process_element(&self, element: &scraper::ElementRef) -> String {
        use scraper::Node;

        let mut result = String::new();

        for child in element.children() {
            match child.value() {
                Node::Text(text) => {
                    let text_content = text.trim();
                    if !text_content.is_empty() {
                        result.push_str(text_content);
                        result.push(' ');
                    }
                }
                Node::Element(elem) => {
                    let tag_name = elem.name();

                    // Create ElementRef for child
                    if let Some(child_elem) = scraper::ElementRef::wrap(child) {
                        match tag_name {
                            "h1" => result
                                .push_str(&format!("# {}\n\n", child_elem.inner_html().trim())),
                            "h2" => result
                                .push_str(&format!("## {}\n\n", child_elem.inner_html().trim())),
                            "h3" => result
                                .push_str(&format!("### {}\n\n", child_elem.inner_html().trim())),
                            "h4" => result
                                .push_str(&format!("#### {}\n\n", child_elem.inner_html().trim())),
                            "h5" => result
                                .push_str(&format!("##### {}\n\n", child_elem.inner_html().trim())),
                            "h6" => result.push_str(&format!(
                                "###### {}\n\n",
                                child_elem.inner_html().trim()
                            )),
                            "p" => result.push_str(&format!(
                                "{}\n\n",
                                child_elem.text().collect::<String>().trim()
                            )),
                            "a" => {
                                if let Some(href) = elem.attr("href") {
                                    result.push_str(&format!(
                                        "[{}]({})",
                                        child_elem.text().collect::<String>(),
                                        href
                                    ));
                                } else {
                                    result.push_str(&child_elem.text().collect::<String>());
                                }
                            }
                            "strong" | "b" => result.push_str(&format!(
                                "**{}**",
                                child_elem.text().collect::<String>()
                            )),
                            "em" | "i" => result
                                .push_str(&format!("*{}*", child_elem.text().collect::<String>())),
                            "code" => result
                                .push_str(&format!("`{}`", child_elem.text().collect::<String>())),
                            "pre" => result.push_str(&format!(
                                "\n```\n{}\n```\n\n",
                                child_elem.text().collect::<String>()
                            )),
                            "ul" | "ol" => {
                                result.push_str(&self.process_list(&child_elem, tag_name == "ol"));
                                result.push_str("\n");
                            }
                            "li" => {} // Handled by process_list
                            "br" => result.push_str("\n"),
                            "hr" => result.push_str("\n---\n\n"),
                            _ => result.push_str(&self.process_element(&child_elem)),
                        }
                    }
                }
                _ => {}
            }
        }

        result
    }

    /// Process list elements
    fn process_list(&self, element: &scraper::ElementRef, ordered: bool) -> String {
        use scraper::Selector;

        let mut result = String::new();

        if let Ok(li_selector) = Selector::parse("li") {
            for (idx, li) in element.select(&li_selector).enumerate() {
                let marker = if ordered {
                    format!("{}. ", idx + 1)
                } else {
                    "- ".to_string()
                };
                result.push_str(&format!(
                    "{}{}\n",
                    marker,
                    li.text().collect::<String>().trim()
                ));
            }
        }

        result
    }
}

impl Default for HtmlConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentConverter for HtmlConverter {
    fn supported_formats(&self) -> Vec<FileFormat> {
        vec![FileFormat::Html]
    }

    fn output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::Markdown {
                split_pages: false,
                optimize_for_llm: true,
            },
            OutputFormat::Json {
                structured: true,
                include_metadata: true,
            },
        ]
    }

    async fn convert(
        &self,
        input: &Path,
        output_format: OutputFormat,
        _options: ConversionOptions,
    ) -> Result<ConversionResult> {
        eprintln!("🔄 HTML Conversion (Pure Rust)");
        eprintln!("   HTML → Semantic Parsing → {:?}", output_format);
        eprintln!();

        // Read HTML file
        let html_content = fs::read_to_string(input).await?;

        // Convert to requested format
        let output_data = match output_format {
            OutputFormat::Markdown { .. } => {
                eprintln!("📝 Converting to Markdown...");
                let markdown = self.html_to_markdown(&html_content)?;
                markdown.into_bytes()
            }
            OutputFormat::Json { .. } => {
                eprintln!("📝 Converting to JSON...");
                // Simple JSON with raw HTML and extracted text
                let markdown = self.html_to_markdown(&html_content)?;
                let json = serde_json::json!({
                    "html": {
                        "raw": html_content,
                        "markdown": markdown,
                        "length": html_content.len(),
                    }
                });
                serde_json::to_string_pretty(&json)?.into_bytes()
            }
            _ => {
                return Err(crate::TransmutationError::UnsupportedFormat(format!(
                    "Output format {:?} not supported for HTML",
                    output_format
                )));
            }
        };

        let output_size = output_data.len() as u64;
        let input_size = fs::metadata(input).await?.len();

        eprintln!("✅ HTML conversion complete!");

        Ok(ConversionResult {
            input_path: input.to_path_buf(),
            input_format: FileFormat::Html,
            output_format,
            content: vec![ConversionOutput {
                page_number: 1,
                data: output_data,
                metadata: OutputMetadata {
                    size_bytes: output_size,
                    chunk_count: 1,
                    token_count: None,
                },
            }],
            metadata: crate::types::DocumentMetadata {
                title: None,
                author: None,
                created: None,
                modified: None,
                page_count: 1,
                language: None,
                custom: std::collections::HashMap::new(),
            },
            statistics: crate::types::ConversionStatistics {
                input_size_bytes: input_size,
                output_size_bytes: output_size,
                duration: std::time::Duration::from_secs(0),
                pages_processed: 1,
                tables_extracted: 0,
                images_extracted: 0,
                cache_hit: false,
            },
        })
    }

    fn metadata(&self) -> ConverterMetadata {
        ConverterMetadata {
            name: "HTML Converter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "HTML to Markdown converter using semantic parsing (pure Rust)"
                .to_string(),
            external_deps: vec![],
        }
    }
}
