//! Heuristic table detection for PDFs
//!
//! This module provides algorithms to detect tables in extracted text based on
//! alignment patterns, spacing, and structural cues.

/// Detected table structure
#[derive(Debug, Clone)]
pub struct DetectedTable {
    /// Table rows
    pub rows: Vec<Vec<String>>,
    /// Column count
    pub column_count: usize,
    /// Whether the first row is a header
    pub has_header: bool,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
}

/// Table detector using heuristic algorithms
#[derive(Debug)]
pub struct TableDetector {
    /// Minimum confidence threshold
    min_confidence: f32,
    /// Minimum columns for a table
    min_columns: usize,
    /// Minimum rows for a table
    min_rows: usize,
}

impl TableDetector {
    /// Create a new table detector
    pub fn new() -> Self {
        Self {
            min_confidence: 0.6,
            min_columns: 2,
            min_rows: 2,
        }
    }

    /// Set minimum confidence threshold
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Detect tables in text using multiple heuristics
    pub fn detect_tables(&self, text: &str) -> Vec<DetectedTable> {
        let mut tables = Vec::new();

        // Try different detection strategies
        tables.extend(self.detect_pipe_delimited_tables(text));
        tables.extend(self.detect_whitespace_aligned_tables(text));
        tables.extend(self.detect_tab_separated_tables(text));

        // Filter by confidence and size
        tables
            .into_iter()
            .filter(|t| {
                t.confidence >= self.min_confidence
                    && t.column_count >= self.min_columns
                    && t.rows.len() >= self.min_rows
            })
            .collect()
    }

    /// Detect pipe-delimited tables (|Col1|Col2|)
    fn detect_pipe_delimited_tables(&self, text: &str) -> Vec<DetectedTable> {
        let mut tables = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            if let Some(table_end) = self.find_pipe_table_end(&lines[i..]) {
                if let Some(table) = self.parse_pipe_table(&lines[i..=i + table_end]) {
                    tables.push(table);
                    i += table_end + 1;
                    continue;
                }
            }
            i += 1;
        }

        tables
    }

    /// Find the end of a pipe-delimited table
    fn find_pipe_table_end(&self, lines: &[&str]) -> Option<usize> {
        let mut end = 0;
        let mut found_separator = false;

        for (i, line) in lines.iter().enumerate() {
            let pipe_count = line.matches('|').count();

            // Check for separator line (| --- | --- |)
            if line.contains("---") && pipe_count >= 2 {
                found_separator = true;
                end = i;
                continue;
            }

            // Check if line has consistent pipe count
            if pipe_count >= 3 && found_separator {
                end = i;
            } else if found_separator {
                break;
            }
        }

        if end > 0 { Some(end) } else { None }
    }

    /// Parse a pipe-delimited table
    fn parse_pipe_table(&self, lines: &[&str]) -> Option<DetectedTable> {
        let mut rows = Vec::new();
        let mut has_header = false;

        for (i, line) in lines.iter().enumerate() {
            // Skip separator lines
            if line.contains("---") {
                has_header = i > 0;
                continue;
            }

            let cells: Vec<String> = line
                .split('|')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect();

            if !cells.is_empty() {
                rows.push(cells);
            }
        }

        if rows.len() < self.min_rows {
            return None;
        }

        let column_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);

        Some(DetectedTable {
            rows,
            column_count,
            has_header,
            confidence: 0.95, // High confidence for pipe-delimited
        })
    }

    /// Detect whitespace-aligned tables
    fn detect_whitespace_aligned_tables(&self, text: &str) -> Vec<DetectedTable> {
        let mut tables = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            if let Some((table_end, column_positions)) = self.find_aligned_table_end(&lines[i..]) {
                if let Some(table) =
                    self.parse_aligned_table(&lines[i..=i + table_end], &column_positions)
                {
                    tables.push(table);
                    i += table_end + 1;
                    continue;
                }
            }
            i += 1;
        }

        tables
    }

    /// Find aligned table by detecting consistent column positions
    fn find_aligned_table_end(&self, lines: &[&str]) -> Option<(usize, Vec<usize>)> {
        if lines.len() < self.min_rows {
            return None;
        }

        // Analyze first few lines to find column positions
        let sample_size = lines.len().min(5);
        let column_positions = self.detect_column_positions(&lines[..sample_size])?;

        if column_positions.len() < self.min_columns {
            return None;
        }

        // Find where the table ends
        let mut end = 0;
        for (i, line) in lines.iter().enumerate() {
            if self.line_matches_columns(line, &column_positions) {
                end = i;
            } else if i > self.min_rows {
                break;
            }
        }

        if end >= self.min_rows - 1 {
            Some((end, column_positions))
        } else {
            None
        }
    }

    /// Detect column positions by finding consistent word boundaries
    fn detect_column_positions(&self, lines: &[&str]) -> Option<Vec<usize>> {
        // Find positions where words consistently start
        let mut position_votes: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        for line in lines {
            let mut in_word = false;
            for (pos, ch) in line.chars().enumerate() {
                if ch.is_whitespace() {
                    in_word = false;
                } else if !in_word {
                    *position_votes.entry(pos).or_insert(0) += 1;
                    in_word = true;
                }
            }
        }

        // Keep positions that appear in most lines
        let threshold = (lines.len() as f32 * 0.7) as usize;
        let mut positions: Vec<usize> = position_votes
            .into_iter()
            .filter(|(_, votes)| *votes >= threshold)
            .map(|(pos, _)| pos)
            .collect();

        positions.sort_unstable();

        if positions.len() >= self.min_columns {
            Some(positions)
        } else {
            None
        }
    }

    /// Check if a line matches detected column positions
    fn line_matches_columns(&self, line: &str, positions: &[usize]) -> bool {
        let mut matches = 0;
        let mut in_word = false;

        for (pos, ch) in line.chars().enumerate() {
            if ch.is_whitespace() {
                in_word = false;
            } else if !in_word {
                // Check if position is near any column position (within 2 chars)
                if positions.iter().any(|&col_pos| pos.abs_diff(col_pos) <= 2) {
                    matches += 1;
                }
                in_word = true;
            }
        }

        // At least 50% of columns should match
        matches >= positions.len() / 2
    }

    /// Parse an aligned table
    fn parse_aligned_table(
        &self,
        lines: &[&str],
        column_positions: &[usize],
    ) -> Option<DetectedTable> {
        let mut rows = Vec::new();

        for line in lines {
            let mut cells = Vec::new();
            let line_chars: Vec<char> = line.chars().collect();

            for i in 0..column_positions.len() {
                let start = column_positions[i].min(line_chars.len());
                let end = column_positions
                    .get(i + 1)
                    .copied()
                    .unwrap_or(line_chars.len())
                    .min(line_chars.len());

                if start < line_chars.len() {
                    let cell: String = line_chars[start..end].iter().collect();
                    cells.push(cell.trim().to_string());
                }
            }

            if !cells.is_empty() {
                rows.push(cells);
            }
        }

        if rows.is_empty() {
            return None;
        }

        // Check if first row looks like a header (shorter cells, capitalized)
        let has_header = rows.get(0).map_or(false, |row| {
            row.iter().all(|cell| {
                cell.len() < 30 && (cell.is_empty() || cell.chars().next().unwrap().is_uppercase())
            })
        });

        Some(DetectedTable {
            rows,
            column_count: column_positions.len(),
            has_header,
            confidence: 0.7, // Medium confidence for aligned tables
        })
    }

    /// Detect tab-separated tables
    fn detect_tab_separated_tables(&self, text: &str) -> Vec<DetectedTable> {
        let mut tables = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            if let Some(table_end) = self.find_tab_table_end(&lines[i..]) {
                if let Some(table) = self.parse_tab_table(&lines[i..=i + table_end]) {
                    tables.push(table);
                    i += table_end + 1;
                    continue;
                }
            }
            i += 1;
        }

        tables
    }

    /// Find end of tab-separated table
    fn find_tab_table_end(&self, lines: &[&str]) -> Option<usize> {
        let mut end = 0;
        let expected_tabs = lines.get(0)?.matches('\t').count();

        if expected_tabs < self.min_columns - 1 {
            return None;
        }

        for (i, line) in lines.iter().enumerate() {
            let tab_count = line.matches('\t').count();
            if tab_count >= expected_tabs - 1 && tab_count <= expected_tabs + 1 {
                end = i;
            } else if i > 0 {
                break;
            }
        }

        if end >= self.min_rows - 1 {
            Some(end)
        } else {
            None
        }
    }

    /// Parse tab-separated table
    fn parse_tab_table(&self, lines: &[&str]) -> Option<DetectedTable> {
        let rows: Vec<Vec<String>> = lines
            .iter()
            .map(|line| line.split('\t').map(|s| s.trim().to_string()).collect())
            .filter(|row: &Vec<String>| !row.is_empty() && row.iter().any(|s| !s.is_empty()))
            .collect();

        if rows.len() < self.min_rows {
            return None;
        }

        let column_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);

        Some(DetectedTable {
            rows,
            column_count,
            has_header: true, // Assume first row is header for TSV
            confidence: 0.85,
        })
    }
}

impl Default for TableDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_delimited_table() {
        let detector = TableDetector::new();
        let text = r"
| Name | Age | City |
| --- | --- | --- |
| Alice | 30 | NYC |
| Bob | 25 | LA |
";

        let tables = detector.detect_tables(text);
        assert!(!tables.is_empty());
        assert_eq!(tables[0].column_count, 3);
        assert!(tables[0].has_header);
    }

    #[test]
    fn test_tab_separated_table() {
        let detector = TableDetector::new();
        let text = "Name\tAge\tCity\nAlice\t30\tNYC\nBob\t25\tLA";

        let tables = detector.detect_tables(text);
        assert!(!tables.is_empty());
        assert_eq!(tables[0].column_count, 3);
    }

    #[test]
    fn test_no_table() {
        let detector = TableDetector::new();
        let text = "This is just regular text with no table structure.";

        let tables = detector.detect_tables(text);
        assert!(tables.is_empty());
    }

    #[test]
    fn test_confidence_threshold() {
        let detector = TableDetector::new().with_confidence(0.9);
        assert_eq!(detector.min_confidence, 0.9);
    }
}
