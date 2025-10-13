/// Integration tests for complete document processing pipeline
/// 
/// Tests the full flow: FFI → Parser → Assembler → Hierarchy → Serializer
#[cfg(all(test, feature = "docling-ffi", feature = "pdf"))]
mod pipeline_tests {
    use transmutation::document::{
        DoclingDocument, DocItem, TextItem, SectionHeaderItem, DocItemLabel,
        PageAssembler, PageAssemblerOptions, HierarchyBuilder, MarkdownSerializer,
    };
    
    #[test]
    fn test_page_assembler_basic() {
        // Test assembler with empty input
        let assembler = PageAssembler::new(PageAssemblerOptions::default());
        let result = assembler.assemble(&[]).unwrap();
        assert_eq!(result.len(), 0);
    }
    
    #[test]
    fn test_hierarchy_builder_basic() {
        // Create sample document items
        let items = vec![
            DocItem::Title(TextItem {
                text: "Test Document".to_string(),
                formatting: None,
                label: DocItemLabel::Title,
            }),
            DocItem::Paragraph(TextItem {
                text: "This is a paragraph.".to_string(),
                formatting: None,
                label: DocItemLabel::Paragraph,
            }),
        ];
        
        let builder = HierarchyBuilder::new();
        let doc = builder.build("test.pdf".to_string(), items).unwrap();
        
        assert_eq!(doc.name, "test.pdf");
        assert_eq!(doc.items.len(), 2);
    }
    
    #[test]
    fn test_hierarchy_builder_section_levels() {
        // Test section level adjustment
        let items = vec![
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
        
        let builder = HierarchyBuilder::new();
        let doc = builder.build("test.pdf".to_string(), items).unwrap();
        
        // Check that the second section was adjusted
        if let DocItem::SectionHeader(ref header) = doc.items[1] {
            assert_eq!(header.level, 2); // Should be corrected from 5 to 2
        } else {
            panic!("Expected SectionHeader");
        }
    }
    
    #[test]
    fn test_markdown_serializer_title() {
        let serializer = MarkdownSerializer::new();
        
        let doc = DoclingDocument {
            name: "test.pdf".to_string(),
            items: vec![
                DocItem::Title(TextItem {
                    text: "Test Title".to_string(),
                    formatting: None,
                    label: DocItemLabel::Title,
                }),
            ],
            items_by_ref: std::collections::HashMap::new(),
        };
        
        let markdown = serializer.serialize(&doc).unwrap();
        assert!(markdown.contains("# Test Title"));
    }
    
    #[test]
    fn test_markdown_serializer_sections() {
        let serializer = MarkdownSerializer::new();
        
        let doc = DoclingDocument {
            name: "test.pdf".to_string(),
            items: vec![
                DocItem::SectionHeader(SectionHeaderItem {
                    text: "Level 1".to_string(),
                    level: 1,
                    formatting: None,
                }),
                DocItem::SectionHeader(SectionHeaderItem {
                    text: "Level 2".to_string(),
                    level: 2,
                    formatting: None,
                }),
            ],
            items_by_ref: std::collections::HashMap::new(),
        };
        
        let markdown = serializer.serialize(&doc).unwrap();
        assert!(markdown.contains("## Level 1"));
        assert!(markdown.contains("### Level 2"));
    }
    
    #[test]
    fn test_markdown_serializer_complete_doc() {
        let serializer = MarkdownSerializer::new();
        
        let doc = DoclingDocument {
            name: "test.pdf".to_string(),
            items: vec![
                DocItem::Title(TextItem {
                    text: "Document Title".to_string(),
                    formatting: None,
                    label: DocItemLabel::Title,
                }),
                DocItem::SectionHeader(SectionHeaderItem {
                    text: "Introduction".to_string(),
                    level: 1,
                    formatting: None,
                }),
                DocItem::Paragraph(TextItem {
                    text: "This is the introduction paragraph.".to_string(),
                    formatting: None,
                    label: DocItemLabel::Paragraph,
                }),
                DocItem::SectionHeader(SectionHeaderItem {
                    text: "Methods".to_string(),
                    level: 1,
                    formatting: None,
                }),
                DocItem::Paragraph(TextItem {
                    text: "This describes the methods.".to_string(),
                    formatting: None,
                    label: DocItemLabel::Paragraph,
                }),
            ],
            items_by_ref: std::collections::HashMap::new(),
        };
        
        let markdown = serializer.serialize(&doc).unwrap();
        
        // Check structure
        assert!(markdown.contains("# Document Title"));
        assert!(markdown.contains("## Introduction"));
        assert!(markdown.contains("## Methods"));
        assert!(markdown.contains("This is the introduction paragraph."));
        assert!(markdown.contains("This describes the methods."));
        
        // Check order
        let title_pos = markdown.find("# Document Title").unwrap();
        let intro_pos = markdown.find("## Introduction").unwrap();
        let methods_pos = markdown.find("## Methods").unwrap();
        
        assert!(title_pos < intro_pos);
        assert!(intro_pos < methods_pos);
    }
    
    #[test]
    fn test_full_pipeline_integration() {
        // Test complete pipeline without FFI (using mock data)
        let items = vec![
            DocItem::Title(TextItem {
                text: "Research Paper".to_string(),
                formatting: None,
                label: DocItemLabel::Title,
            }),
            DocItem::SectionHeader(SectionHeaderItem {
                text: "Abstract".to_string(),
                level: 1,
                formatting: None,
            }),
            DocItem::Paragraph(TextItem {
                text: "This paper presents a novel approach.".to_string(),
                formatting: None,
                label: DocItemLabel::Paragraph,
            }),
        ];
        
        // Step 1: Build hierarchy
        let builder = HierarchyBuilder::new();
        let doc = builder.build("paper.pdf".to_string(), items).unwrap();
        
        assert_eq!(doc.items.len(), 3);
        
        // Step 2: Serialize
        let serializer = MarkdownSerializer::new();
        let markdown = serializer.serialize(&doc).unwrap();
        
        // Verify output
        assert!(markdown.contains("# Research Paper"));
        assert!(markdown.contains("## Abstract"));
        assert!(markdown.contains("This paper presents a novel approach."));
        
        // Check clean formatting (no triple newlines)
        assert!(!markdown.contains("\n\n\n"));
    }
}

#[cfg(all(test, feature = "docling-ffi"))]
mod component_tests {
    use transmutation::document::text_utils::*;
    
    #[test]
    fn test_text_sanitizer_hyphens() {
        let sanitizer = TextSanitizer::new();
        let text = "This is a hyphen-\nated word in a sen-\ntence.";
        let result = sanitizer.sanitize(text);
        
        assert_eq!(result, "This is a hyphenated word in a sentence.");
    }
    
    #[test]
    fn test_text_sanitizer_ligatures() {
        let sanitizer = TextSanitizer::new();
        let text = "The ﬁle with ﬀ and ﬂ ligatures.";
        let result = sanitizer.sanitize(text);
        
        assert_eq!(result, "The file with ff and fl ligatures.");
    }
    
    #[test]
    fn test_text_sanitizer_special_chars() {
        let sanitizer = TextSanitizer::new();
        let text = "Price: $100⁄month — "special" offer";
        let result = sanitizer.sanitize(text);
        
        assert_eq!(result, "Price: $100/month - \"special\" offer");
    }
    
    #[test]
    fn test_heading_detection() {
        assert!(is_likely_heading("1. Introduction"));
        assert!(is_likely_heading("CHAPTER ONE"));
        assert!(is_likely_heading("Methods and Results"));
        
        assert!(!is_likely_heading("This is a regular sentence with normal text."));
        assert!(!is_likely_heading("This is a very long paragraph that goes on and on and should not be detected as a heading."));
    }
    
    #[test]
    fn test_section_number_extraction() {
        assert_eq!(extract_section_number("1.2.3 Methods"), Some("1.2.3".to_string()));
        assert_eq!(extract_section_number("5. Results"), Some("5".to_string()));
        assert_eq!(extract_section_number("Regular text"), None);
    }
    
    #[test]
    fn test_section_level_calculation() {
        assert_eq!(calculate_section_level("1"), 1);
        assert_eq!(calculate_section_level("1.2"), 2);
        assert_eq!(calculate_section_level("1.2.3"), 3);
        assert_eq!(calculate_section_level("1.2.3.4.5"), 5);
    }
}

