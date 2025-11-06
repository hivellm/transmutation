//! Text sanitization and normalization utilities
//!
//! Based on docling's page_assemble_model.py text processing

#![allow(missing_docs)]

use once_cell::sync::Lazy;
use regex::Regex;

/// Hyphen patterns for joining across line breaks
static HYPHEN_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)-\s*\n\s*(\w+)").unwrap());

/// Multiple whitespace pattern
static MULTI_SPACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

/// Line break pattern for joining
static LINE_BREAK_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"([^\n])\n([^\n])").unwrap());

/// Character normalization map (Unicode → ASCII/common forms)
/// Reference: docling/models/page_assemble_model.py lines 34-65
/// Extended with mathematical symbols, Greek letters, and special characters
const CHAR_NORMALIZATION_MAP: &[(&str, &str)] = &[
    // Fractions and special symbols
    ("⁄", "/"),
    ("∕", "/"),
    // Quotes
    ("'", "'"),         // U+2018 Left single quotation mark
    ("'", "'"),         // U+2019 Right single quotation mark
    ("\u{201C}", "\""), // U+201C Left double quotation mark
    ("\u{201D}", "\""), // U+201D Right double quotation mark
    ("\u{201E}", "\""), // U+201E Double low-9 quotation mark
    ("\u{201F}", "\""), // U+201F Double high-reversed-9 quotation mark
    ("‹", "<"),         // single left-pointing angle quotation mark
    ("›", ">"),         // single right-pointing angle quotation mark
    ("«", "<<"),        // left-pointing double angle quotation mark
    ("»", ">>"),        // right-pointing double angle quotation mark
    // Dashes and hyphens
    ("–", "-"), // en dash
    ("—", "-"), // em dash
    ("‐", "-"), // hyphen
    ("‑", "-"), // non-breaking hyphen
    ("−", "-"), // minus sign
    ("‒", "-"), // figure dash
    ("―", "-"), // horizontal bar
    // Bullets
    ("•", "·"),
    ("‣", "·"),
    ("⁃", "·"),
    ("◦", "o"),
    ("▪", "·"),
    ("▫", "o"),
    // Ellipsis
    ("…", "..."),
    // Spaces
    (" ", " "), // non-breaking space
    (" ", " "), // thin space
    (" ", " "), // hair space
    (" ", " "), // en space
    (" ", " "), // em space
    // Math symbols - basic
    ("×", "x"),
    ("÷", "/"),
    ("±", "+/-"),
    ("∓", "-/+"),
    // Math symbols - comparison
    ("≤", "<="),
    ("≥", ">="),
    ("≠", "!="),
    ("≈", "~="),
    ("≡", "==="),
    ("≢", "!=="),
    // Math symbols - advanced
    ("∞", "infinity"),
    ("∫", "integral"),
    ("∑", "sum"),
    ("∏", "product"),
    ("√", "sqrt"),
    ("∛", "cbrt"),
    ("∜", "fourthrt"),
    ("∂", "d"), // partial derivative
    ("∆", "delta"),
    ("∇", "nabla"),
    // Arrows
    ("→", "->"),
    ("←", "<-"),
    ("↑", "^"),
    ("↓", "v"),
    ("↔", "<->"),
    ("⇒", "=>"),
    ("⇐", "<="),
    ("⇔", "<=>"),
    ("↦", "|->"),
    // Superscripts (for exponents in scientific notation)
    ("⁰", "0"),
    ("¹", "1"),
    ("²", "2"),
    ("³", "3"),
    ("⁴", "4"),
    ("⁵", "5"),
    ("⁶", "6"),
    ("⁷", "7"),
    ("⁸", "8"),
    ("⁹", "9"),
    ("⁺", "+"),
    ("⁻", "-"),
    ("⁼", "="),
    ("⁽", "("),
    ("⁾", ")"),
    // Subscripts
    ("₀", "0"),
    ("₁", "1"),
    ("₂", "2"),
    ("₃", "3"),
    ("₄", "4"),
    ("₅", "5"),
    ("₆", "6"),
    ("₇", "7"),
    ("₈", "8"),
    ("₉", "9"),
    ("₊", "+"),
    ("₋", "-"),
    ("₌", "="),
    ("₍", "("),
    ("₎", ")"),
    // Greek letters (common in scientific papers)
    ("α", "alpha"),
    ("β", "beta"),
    ("γ", "gamma"),
    ("δ", "delta"),
    ("ε", "epsilon"),
    ("ζ", "zeta"),
    ("η", "eta"),
    ("θ", "theta"),
    ("ι", "iota"),
    ("κ", "kappa"),
    ("λ", "lambda"),
    ("μ", "mu"),
    ("ν", "nu"),
    ("ξ", "xi"),
    ("ο", "omicron"),
    ("π", "pi"),
    ("ρ", "rho"),
    ("σ", "sigma"),
    ("τ", "tau"),
    ("υ", "upsilon"),
    ("φ", "phi"),
    ("χ", "chi"),
    ("ψ", "psi"),
    ("ω", "omega"),
    // Capital Greek letters
    ("Α", "Alpha"),
    ("Β", "Beta"),
    ("Γ", "Gamma"),
    ("Δ", "Delta"),
    ("Ε", "Epsilon"),
    ("Ζ", "Zeta"),
    ("Η", "Eta"),
    ("Θ", "Theta"),
    ("Ι", "Iota"),
    ("Κ", "Kappa"),
    ("Λ", "Lambda"),
    ("Μ", "Mu"),
    ("Ν", "Nu"),
    ("Ξ", "Xi"),
    ("Ο", "Omicron"),
    ("Π", "Pi"),
    ("Ρ", "Rho"),
    ("Σ", "Sigma"),
    ("Τ", "Tau"),
    ("Υ", "Upsilon"),
    ("Φ", "Phi"),
    ("Χ", "Chi"),
    ("Ψ", "Psi"),
    ("Ω", "Omega"),
    // Ligatures
    ("ﬁ", "fi"),
    ("ﬂ", "fl"),
    ("ﬀ", "ff"),
    ("ﬃ", "ffi"),
    ("ﬄ", "ffl"),
    ("ﬅ", "ft"),
    ("ﬆ", "st"),
    // Special symbols
    ("©", "(c)"),
    ("®", "(R)"),
    ("™", "(TM)"),
    ("°", " degrees"),
    ("§", "section"),
    ("¶", "paragraph"),
    ("†", "+"),  // dagger
    ("‡", "++"), // double dagger
    ("¢", "cents"),
    ("£", "GBP"),
    ("¥", "JPY"),
    ("€", "EUR"),
];

/// Text sanitizer for document text
#[derive(Debug)]
pub struct TextSanitizer {
    join_hyphens: bool,
    join_lines: bool,
    normalize_chars: bool,
    normalize_whitespace: bool,
}

impl TextSanitizer {
    /// Create new sanitizer with all options enabled
    pub fn new() -> Self {
        Self {
            join_hyphens: true,
            join_lines: true,
            normalize_chars: true,
            normalize_whitespace: true,
        }
    }

    /// Create sanitizer with custom options
    pub fn with_options(
        join_hyphens: bool,
        join_lines: bool,
        normalize_chars: bool,
        normalize_whitespace: bool,
    ) -> Self {
        Self {
            join_hyphens,
            join_lines,
            normalize_chars,
            normalize_whitespace,
        }
    }

    /// Sanitize text with all configured options
    pub fn sanitize(&self, text: &str) -> String {
        let mut result = text.to_string();

        if self.normalize_chars {
            result = self.normalize_characters(&result);
        }

        if self.join_hyphens {
            result = self.join_hyphenated_words(&result);
        }

        if self.join_lines {
            result = self.join_lines_with_space(&result);
        }

        if self.normalize_whitespace {
            result = self.normalize_whitespace_chars(&result);
        }

        result.trim().to_string()
    }

    /// Join hyphenated words across line breaks
    /// "word-\nword" → "wordword"
    fn join_hyphenated_words(&self, text: &str) -> String {
        HYPHEN_PATTERN.replace_all(text, "$1$2").to_string()
    }

    /// Join lines with spaces (but preserve paragraph breaks)
    /// "line1\nline2" → "line1 line2"
    /// But "line1\n\nline2" stays as is
    fn join_lines_with_space(&self, text: &str) -> String {
        LINE_BREAK_PATTERN.replace_all(text, "$1 $2").to_string()
    }

    /// Normalize special characters to common forms
    fn normalize_characters(&self, text: &str) -> String {
        let mut result = text.to_string();

        for (from, to) in CHAR_NORMALIZATION_MAP {
            result = result.replace(from, to);
        }

        result
    }

    /// Normalize multiple whitespace to single space
    fn normalize_whitespace_chars(&self, text: &str) -> String {
        MULTI_SPACE_PATTERN.replace_all(text, " ").to_string()
    }
}

impl Default for TextSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick sanitize function with default options
pub fn sanitize_text(text: &str) -> String {
    TextSanitizer::new().sanitize(text)
}

/// Join text from multiple cells/lines with proper spacing
///
/// This handles spacing based on position (if available) or falls back
/// to simple joining with spaces.
pub fn join_text_cells(texts: &[&str], add_spaces: bool) -> String {
    if texts.is_empty() {
        return String::new();
    }

    if add_spaces {
        texts.join(" ")
    } else {
        texts.concat()
    }
}

/// Detect if text is likely a heading/title based on patterns
///
/// Heuristics:
/// - Short length (< 100 chars)
/// - No ending punctuation (., ?, !)
/// - Mostly capitalized or all caps
/// - Contains numbers (section numbers)
pub fn is_likely_heading(text: &str) -> bool {
    let text = text.trim();

    if text.is_empty() || text.len() > 100 {
        return false;
    }

    // Check if ends with sentence-ending punctuation
    if text.ends_with('.') || text.ends_with('?') || text.ends_with('!') {
        return false;
    }

    // Check capitalization
    let uppercase_ratio = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .filter(|c| c.is_uppercase())
        .count() as f32
        / text.chars().filter(|c| c.is_alphabetic()).count().max(1) as f32;

    // High uppercase ratio suggests heading
    if uppercase_ratio > 0.7 {
        return true;
    }

    // Check for section numbers (1.2, 1.2.3, etc.)
    let section_number_pattern = Regex::new(r"^\d+(\.\d+)*\.?\s").unwrap();
    if section_number_pattern.is_match(text) {
        return true;
    }

    false
}

/// Extract section number from heading text
/// "1.2.3 Introduction" → Some("1.2.3")
pub fn extract_section_number(text: &str) -> Option<String> {
    let section_pattern = Regex::new(r"^(\d+(\.\d+)*)\.?\s").unwrap();

    section_pattern
        .captures(text)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

/// Calculate section level from number
/// "1" → 1, "1.2" → 2, "1.2.3" → 3
pub fn calculate_section_level(section_number: &str) -> usize {
    section_number.split('.').count()
}

/// Remove common PDF artifacts
/// - Zero-width spaces
/// - Soft hyphens
/// - Control characters
pub fn remove_pdf_artifacts(text: &str) -> String {
    text.chars()
        .filter(|&c| {
            // Remove zero-width and control characters
            !matches!(
                c,
                '\u{200B}'  // zero-width space
                | '\u{200C}'  // zero-width non-joiner
                | '\u{200D}'  // zero-width joiner
                | '\u{FEFF}'  // zero-width no-break space
                | '\u{00AD}'  // soft hyphen
                | '\0'..='\u{001F}' // control characters (except newline/tab)
            ) || c == '\n'
                || c == '\t'
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyphen_joining() {
        let sanitizer = TextSanitizer::new();
        let text = "This is a hyphen-\nated word.";
        let result = sanitizer.sanitize(text);
        assert_eq!(result, "This is a hyphenated word.");
    }

    #[test]
    fn test_line_joining() {
        let sanitizer = TextSanitizer::new();
        let text = "Line one\nLine two";
        let result = sanitizer.sanitize(text);
        assert_eq!(result, "Line one Line two");
    }

    #[test]
    fn test_character_normalization() {
        let sanitizer = TextSanitizer::new();
        let text = "Price: $100⁄month — \u{201C}special\u{201D} offer";
        let result = sanitizer.sanitize(text);
        assert_eq!(result, "Price: $100/month - \"special\" offer");
    }

    #[test]
    fn test_whitespace_normalization() {
        let sanitizer = TextSanitizer::new();
        let text = "Too    many     spaces";
        let result = sanitizer.sanitize(text);
        assert_eq!(result, "Too many spaces");
    }

    #[test]
    fn test_is_likely_heading() {
        assert!(is_likely_heading("1. Introduction"));
        assert!(is_likely_heading("CHAPTER 1"));
        assert!(is_likely_heading("1.2.3 Methods"));
        assert!(!is_likely_heading("This is a regular sentence."));
        assert!(!is_likely_heading(
            "This is a very long text that goes on and on and definitely should not be considered a heading because it's way too long."
        ));
    }

    #[test]
    fn test_extract_section_number() {
        assert_eq!(
            extract_section_number("1.2.3 Methods"),
            Some("1.2.3".to_string())
        );
        assert_eq!(
            extract_section_number("1. Introduction"),
            Some("1".to_string())
        );
        assert_eq!(extract_section_number("No number here"), None);
    }

    #[test]
    fn test_calculate_section_level() {
        assert_eq!(calculate_section_level("1"), 1);
        assert_eq!(calculate_section_level("1.2"), 2);
        assert_eq!(calculate_section_level("1.2.3"), 3);
        assert_eq!(calculate_section_level("1.2.3.4"), 4);
    }

    #[test]
    fn test_remove_pdf_artifacts() {
        let text = "Hello\u{200B}World\u{00AD}Test";
        let result = remove_pdf_artifacts(text);
        assert_eq!(result, "HelloWorldTest");
    }

    #[test]
    fn test_ligature_normalization() {
        let sanitizer = TextSanitizer::new();
        let text = "ﬁle with ligatures: ﬀ, ﬁ, ﬂ";
        let result = sanitizer.sanitize(text);
        assert_eq!(result, "file with ligatures: ff, fi, fl");
    }
}
