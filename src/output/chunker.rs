//! Text chunking for LLM context optimization
//!
//! This module provides strategies for splitting text into chunks suitable for
//! embedding generation and LLM processing.

use crate::types::ConversionOptions;

/// Text chunking strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkStrategy {
    /// Fixed token count with overlap
    TokenBased,
    /// Preserve semantic boundaries (paragraphs, sections)
    Semantic,
    /// Sliding window approach
    SlidingWindow,
}

/// A single text chunk with metadata
#[derive(Debug, Clone)]
pub struct TextChunk {
    /// Chunk content
    pub content: String,
    /// Chunk index (0-based)
    pub index: usize,
    /// Start position in original text (character offset)
    pub start_offset: usize,
    /// End position in original text (character offset)
    pub end_offset: usize,
    /// Estimated token count
    pub token_count: usize,
}

/// Text chunker
pub struct Chunker {
    strategy: ChunkStrategy,
    max_chunk_size: usize,
    overlap: usize,
}

impl Chunker {
    /// Create a new chunker with specified strategy
    pub fn new(strategy: ChunkStrategy, max_chunk_size: usize, overlap: usize) -> Self {
        Self {
            strategy,
            max_chunk_size,
            overlap,
        }
    }

    /// Create chunker from conversion options
    pub fn from_options(options: &ConversionOptions) -> Self {
        Self::new(
            ChunkStrategy::Semantic, // Default to semantic
            options.max_chunk_size,
            200, // Default overlap
        )
    }

    /// Split text into chunks
    pub fn chunk(&self, text: &str) -> Vec<TextChunk> {
        match self.strategy {
            ChunkStrategy::TokenBased => self.chunk_token_based(text),
            ChunkStrategy::Semantic => self.chunk_semantic(text),
            ChunkStrategy::SlidingWindow => self.chunk_sliding_window(text),
        }
    }

    /// Token-based chunking with overlap
    fn chunk_token_based(&self, text: &str) -> Vec<TextChunk> {
        let mut chunks = Vec::new();
        let mut current_pos = 0;
        let mut chunk_index = 0;

        while current_pos < text.len() {
            // Calculate chunk size in characters (rough approximation: 4 chars ≈ 1 token)
            let char_limit = self.max_chunk_size * 4;
            let end_pos = (current_pos + char_limit).min(text.len());

            // Find a good break point (end of sentence or paragraph)
            let chunk_end = if end_pos < text.len() {
                self.find_break_point(text, current_pos, end_pos)
            } else {
                end_pos
            };

            let content = text[current_pos..chunk_end].to_string();
            let token_count = self.estimate_tokens(&content);

            chunks.push(TextChunk {
                content,
                index: chunk_index,
                start_offset: current_pos,
                end_offset: chunk_end,
                token_count,
            });

            // Move forward with overlap
            let overlap_chars = self.overlap * 4;
            current_pos = if chunk_end > overlap_chars {
                chunk_end - overlap_chars
            } else {
                chunk_end
            };

            chunk_index += 1;
        }

        chunks
    }

    /// Semantic chunking (preserve paragraph boundaries)
    fn chunk_semantic(&self, text: &str) -> Vec<TextChunk> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut chunk_start = 0;
        let mut chunk_index = 0;

        for paragraph in text.split("\n\n") {
            let paragraph_tokens = self.estimate_tokens(paragraph);
            let current_tokens = self.estimate_tokens(&current_chunk);

            if current_tokens + paragraph_tokens > self.max_chunk_size && !current_chunk.is_empty()
            {
                // Save current chunk
                let chunk_end = chunk_start + current_chunk.len();
                chunks.push(TextChunk {
                    content: current_chunk.trim().to_string(),
                    index: chunk_index,
                    start_offset: chunk_start,
                    end_offset: chunk_end,
                    token_count: current_tokens,
                });

                chunk_index += 1;
                chunk_start = chunk_end;
                current_chunk.clear();
            }

            if !current_chunk.is_empty() {
                current_chunk.push_str("\n\n");
            }
            current_chunk.push_str(paragraph);
        }

        // Add remaining chunk
        if !current_chunk.is_empty() {
            let chunk_end = chunk_start + current_chunk.len();
            let token_count = self.estimate_tokens(&current_chunk);
            chunks.push(TextChunk {
                content: current_chunk.trim().to_string(),
                index: chunk_index,
                start_offset: chunk_start,
                end_offset: chunk_end,
                token_count,
            });
        }

        chunks
    }

    /// Sliding window chunking
    fn chunk_sliding_window(&self, text: &str) -> Vec<TextChunk> {
        let mut chunks = Vec::new();
        let char_limit = self.max_chunk_size * 4;
        let step_size = char_limit - (self.overlap * 4);
        let mut chunk_index = 0;

        let mut current_pos = 0;
        while current_pos < text.len() {
            let end_pos = (current_pos + char_limit).min(text.len());
            let content = text[current_pos..end_pos].to_string();
            let token_count = self.estimate_tokens(&content);

            chunks.push(TextChunk {
                content,
                index: chunk_index,
                start_offset: current_pos,
                end_offset: end_pos,
                token_count,
            });

            current_pos += step_size;
            chunk_index += 1;

            if end_pos >= text.len() {
                break;
            }
        }

        chunks
    }

    /// Find a good break point (end of sentence or paragraph)
    fn find_break_point(&self, text: &str, start: usize, preferred_end: usize) -> usize {
        let search_text = &text[start..preferred_end];

        // Try to find end of sentence
        if let Some(pos) = search_text.rfind(". ") {
            return start + pos + 1;
        }

        // Try to find end of paragraph
        if let Some(pos) = search_text.rfind("\n\n") {
            return start + pos + 2;
        }

        // Try to find newline
        if let Some(pos) = search_text.rfind('\n') {
            return start + pos + 1;
        }

        // Try to find space
        if let Some(pos) = search_text.rfind(' ') {
            return start + pos + 1;
        }

        // No good break point found, use preferred end
        preferred_end
    }

    /// Estimate token count (rough approximation: 4 chars ≈ 1 token)
    fn estimate_tokens(&self, text: &str) -> usize {
        // Simple heuristic: average of 4 characters per token
        // This is a rough approximation; for accurate counting, use tiktoken-rs
        (text.len() + 3) / 4
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new(ChunkStrategy::Semantic, 2048, 200)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_based_chunking() {
        let chunker = Chunker::new(ChunkStrategy::TokenBased, 100, 20);
        let text = "This is a test. ".repeat(50); // ~50 sentences
        let chunks = chunker.chunk(&text);

        assert!(!chunks.is_empty());
        for chunk in &chunks {
            assert!(chunk.token_count <= 100 + 50); // Allow some buffer
        }
    }

    #[test]
    fn test_semantic_chunking() {
        let chunker = Chunker::new(ChunkStrategy::Semantic, 100, 0);
        let text = vec![
            "Paragraph 1. This is the first paragraph.",
            "Paragraph 2. This is the second paragraph.",
            "Paragraph 3. This is the third paragraph.",
        ]
        .join("\n\n");

        let chunks = chunker.chunk(&text);
        assert!(!chunks.is_empty());

        // Each chunk should contain complete paragraphs
        for chunk in &chunks {
            assert!(!chunk.content.trim().is_empty());
        }
    }

    #[test]
    fn test_sliding_window_chunking() {
        let chunker = Chunker::new(ChunkStrategy::SlidingWindow, 50, 10);
        let text = "word ".repeat(200); // 200 words
        let chunks = chunker.chunk(&text);

        assert!(chunks.len() > 1);

        // Check overlap
        for i in 1..chunks.len() {
            let prev_end = &chunks[i - 1].content[chunks[i - 1].content.len().saturating_sub(50)..];
            let curr_start = &chunks[i].content[..50.min(chunks[i].content.len())];

            // There should be some overlap
            assert!(
                prev_end
                    .split_whitespace()
                    .any(|word| curr_start.contains(word)),
                "Expected overlap between chunks"
            );
        }
    }

    #[test]
    fn test_chunk_metadata() {
        let chunker = Chunker::default();
        let text = "Test content. ".repeat(100);
        let chunks = chunker.chunk(&text);

        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.index, i);
            assert!(chunk.start_offset < chunk.end_offset);
            assert!(chunk.token_count > 0);
        }
    }

    #[test]
    fn test_empty_text() {
        let chunker = Chunker::default();
        let chunks = chunker.chunk("");
        assert!(chunks.is_empty() || chunks.len() == 1);
    }

    #[test]
    fn test_token_estimation() {
        let chunker = Chunker::default();
        let text = "This is a test";
        let tokens = chunker.estimate_tokens(text);
        // Rough estimate: 14 chars / 4 ≈ 3-4 tokens
        assert!(tokens >= 3 && tokens <= 5);
    }
}
