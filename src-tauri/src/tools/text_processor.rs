use crate::error::{HoverShellError, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepResult {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOptions {
    pub numeric: bool,
    pub reverse: bool,
    pub case_insensitive: bool,
    pub unique: bool,
    pub field_separator: Option<String>,
    pub field_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SedOptions {
    pub global: bool,
    pub case_insensitive: bool,
    pub extended_regex: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwkOptions {
    pub field_separator: Option<String>,
    pub output_separator: Option<String>,
    pub variables: HashMap<String, String>,
}

pub struct TextProcessor {
    // Configuration
    max_file_size: u64,
    default_encoding: String,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100MB
            default_encoding: "utf-8".to_string(),
        }
    }

    /// Search for patterns in text using grep-like functionality
    pub async fn grep(&self, pattern: &str, files: &[String], options: &GrepOptions) -> Result<Vec<GrepResult>> {
        let mut results = Vec::new();
        let regex_pattern = self.build_regex_pattern(pattern, options)?;

        for file_path in files {
            if let Ok(file_results) = self.grep_file(&regex_pattern, file_path, options).await {
                results.extend(file_results);
            }
        }

        info!("Grep found {} matches for pattern '{}'", results.len(), pattern);
        Ok(results)
    }

    /// Search in a single file
    async fn grep_file(&self, regex: &regex::Regex, file_path: &str, options: &GrepOptions) -> Result<Vec<GrepResult>> {
        let content = fs::read_to_string(file_path).await?;
        let mut results = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if options.case_insensitive {
                // Case insensitive search
                if let Some(mat) = regex.find(&line.to_lowercase()) {
                    results.push(GrepResult {
                        file_path: file_path.to_string(),
                        line_number: line_num + 1,
                        line_content: line.to_string(),
                        match_start: mat.start(),
                        match_end: mat.end(),
                    });
                }
            } else {
                // Case sensitive search
                if let Some(mat) = regex.find(line) {
                    results.push(GrepResult {
                        file_path: file_path.to_string(),
                        line_number: line_num + 1,
                        line_content: line.to_string(),
                        match_start: mat.start(),
                        match_end: mat.end(),
                    });
                }
            }
        }

        Ok(results)
    }

    /// Sort lines of text
    pub async fn sort(&self, input: &str, options: &SortOptions) -> Result<String> {
        let mut lines: Vec<&str> = input.lines().collect();

        // Apply sorting
        if options.numeric {
            lines.sort_by(|a, b| {
                let a_num: f64 = a.parse().unwrap_or(0.0);
                let b_num: f64 = b.parse().unwrap_or(0.0);
                a_num.partial_cmp(&b_num).unwrap_or(std::cmp::Ordering::Equal)
            });
        } else if let Some(field_num) = options.field_number {
            // Sort by specific field
            let separator = options.field_separator.as_deref().unwrap_or(" ");
            lines.sort_by(|a, b| {
                let a_field = a.split(separator).nth(field_num - 1).unwrap_or("");
                let b_field = b.split(separator).nth(field_num - 1).unwrap_or("");
                
                if options.case_insensitive {
                    a_field.to_lowercase().cmp(&b_field.to_lowercase())
                } else {
                    a_field.cmp(b_field)
                }
            });
        } else {
            // Default string sorting
            if options.case_insensitive {
                lines.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            } else {
                lines.sort();
            }
        }

        // Reverse if requested
        if options.reverse {
            lines.reverse();
        }

        // Remove duplicates if requested
        if options.unique {
            lines.dedup();
        }

        Ok(lines.join("\n"))
    }

    /// Apply sed-like text transformations
    pub async fn sed(&self, input: &str, pattern: &str, replacement: &str, options: &SedOptions) -> Result<String> {
        let regex_pattern = if options.extended_regex {
            regex::Regex::new(pattern)?
        } else {
            // Escape special regex characters for basic mode
            let escaped = regex::escape(pattern);
            regex::Regex::new(&escaped)?
        };

        let result = if options.global {
            if options.case_insensitive {
                regex_pattern.replace_all(input, replacement)
            } else {
                regex_pattern.replace_all(input, replacement)
            }
        } else {
            // Replace only first occurrence
            if let Some(first_match) = regex_pattern.find(input) {
                let mut result = input.to_string();
                result.replace_range(first_match.start()..first_match.end(), replacement);
                result
            } else {
                input.to_string()
            }
        };

        Ok(result)
    }

    /// Apply awk-like text processing
    pub async fn awk(&self, input: &str, script: &str, options: &AwkOptions) -> Result<String> {
        // Simple awk implementation for common operations
        let separator = options.field_separator.as_deref().unwrap_or(" ");
        let mut output = Vec::new();

        for line in input.lines() {
            let fields: Vec<&str> = line.split(separator).collect();
            
            // Simple field operations
            if script.contains("$1") {
                if let Some(first_field) = fields.get(0) {
                    output.push(first_field.to_string());
                }
            } else if script.contains("$2") {
                if let Some(second_field) = fields.get(1) {
                    output.push(second_field.to_string());
                }
            } else if script.contains("$NF") {
                if let Some(last_field) = fields.last() {
                    output.push(last_field.to_string());
                }
            } else if script.contains("print") {
                // Simple print operation
                output.push(line.to_string());
            } else {
                // Default: print the line
                output.push(line.to_string());
            }
        }

        Ok(output.join("\n"))
    }

    /// Count lines, words, and characters
    pub async fn wc(&self, input: &str) -> Result<WcResult> {
        let lines = input.lines().count();
        let words = input.split_whitespace().count();
        let chars = input.chars().count();
        let bytes = input.len();

        Ok(WcResult {
            lines,
            words,
            chars,
            bytes,
        })
    }

    /// Remove duplicate lines
    pub async fn uniq(&self, input: &str, case_insensitive: bool) -> Result<String> {
        let mut lines: Vec<&str> = input.lines().collect();
        
        if case_insensitive {
            // Case insensitive deduplication
            let mut seen = std::collections::HashSet::new();
            lines.retain(|line| seen.insert(line.to_lowercase()));
        } else {
            lines.dedup();
        }

        Ok(lines.join("\n"))
    }

    /// Cut specific fields from lines
    pub async fn cut(&self, input: &str, delimiter: &str, fields: &[usize]) -> Result<String> {
        let mut output = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(delimiter).collect();
            let mut selected_parts = Vec::new();

            for &field_num in fields {
                if field_num > 0 && field_num <= parts.len() {
                    selected_parts.push(parts[field_num - 1]);
                }
            }

            output.push(selected_parts.join(delimiter));
        }

        Ok(output.join("\n"))
    }

    /// Join lines with a delimiter
    pub async fn join(&self, input: &str, delimiter: &str) -> Result<String> {
        let lines: Vec<&str> = input.lines().collect();
        Ok(lines.join(delimiter))
    }

    /// Split text into lines
    pub async fn split_lines(&self, input: &str) -> Result<Vec<String>> {
        Ok(input.lines().map(|s| s.to_string()).collect())
    }

    /// Split text into words
    pub async fn split_words(&self, input: &str) -> Result<Vec<String>> {
        Ok(input.split_whitespace().map(|s| s.to_string()).collect())
    }

    /// Convert text to uppercase
    pub async fn to_uppercase(&self, input: &str) -> Result<String> {
        Ok(input.to_uppercase())
    }

    /// Convert text to lowercase
    pub async fn to_lowercase(&self, input: &str) -> Result<String> {
        Ok(input.to_lowercase())
    }

    /// Capitalize first letter of each word
    pub async fn capitalize(&self, input: &str) -> Result<String> {
        let words: Vec<String> = input
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect();
        Ok(words.join(" "))
    }

    /// Reverse text
    pub async fn reverse(&self, input: &str) -> Result<String> {
        Ok(input.chars().rev().collect())
    }

    /// Truncate text to specified length
    pub async fn truncate(&self, input: &str, length: usize, suffix: Option<&str>) -> Result<String> {
        if input.len() <= length {
            return Ok(input.to_string());
        }

        let suffix = suffix.unwrap_or("...");
        let truncate_length = length.saturating_sub(suffix.len());
        let truncated = &input[..truncate_length];
        Ok(format!("{}{}", truncated, suffix))
    }

    /// Remove leading and trailing whitespace
    pub async fn trim(&self, input: &str) -> Result<String> {
        Ok(input.trim().to_string())
    }

    /// Replace all occurrences of a substring
    pub async fn replace(&self, input: &str, from: &str, to: &str) -> Result<String> {
        Ok(input.replace(from, to))
    }

    /// Extract lines matching a pattern
    pub async fn extract_lines(&self, input: &str, pattern: &str, case_insensitive: bool) -> Result<String> {
        let regex_pattern = if case_insensitive {
            regex::Regex::new(&format!("(?i){}", pattern))?
        } else {
            regex::Regex::new(pattern)?
        };

        let matching_lines: Vec<&str> = input
            .lines()
            .filter(|line| regex_pattern.is_match(line))
            .collect();

        Ok(matching_lines.join("\n"))
    }

    /// Remove lines matching a pattern
    pub async fn remove_lines(&self, input: &str, pattern: &str, case_insensitive: bool) -> Result<String> {
        let regex_pattern = if case_insensitive {
            regex::Regex::new(&format!("(?i){}", pattern))?
        } else {
            regex::Regex::new(pattern)?
        };

        let filtered_lines: Vec<&str> = input
            .lines()
            .filter(|line| !regex_pattern.is_match(line))
            .collect();

        Ok(filtered_lines.join("\n"))
    }

    /// Build regex pattern with options
    fn build_regex_pattern(&self, pattern: &str, options: &GrepOptions) -> Result<regex::Regex> {
        let mut regex_pattern = pattern.to_string();

        if options.extended_regex {
            // Use pattern as-is for extended regex
        } else {
            // Escape special characters for basic regex
            regex_pattern = regex::escape(pattern);
        }

        if options.case_insensitive {
            regex_pattern = format!("(?i){}", regex_pattern);
        }

        regex::Regex::new(&regex_pattern)
            .map_err(|e| HoverShellError::TextProcessing(format!("Invalid regex pattern: {}", e)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepOptions {
    pub case_insensitive: bool,
    pub extended_regex: bool,
    pub whole_word: bool,
    pub line_number: bool,
    pub count_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WcResult {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

impl Default for TextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GrepOptions {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            extended_regex: false,
            whole_word: false,
            line_number: false,
            count_only: false,
        }
    }
}

impl Default for SortOptions {
    fn default() -> Self {
        Self {
            numeric: false,
            reverse: false,
            case_insensitive: false,
            unique: false,
            field_separator: None,
            field_number: None,
        }
    }
}

impl Default for SedOptions {
    fn default() -> Self {
        Self {
            global: false,
            case_insensitive: false,
            extended_regex: false,
        }
    }
}

impl Default for AwkOptions {
    fn default() -> Self {
        Self {
            field_separator: None,
            output_separator: None,
            variables: HashMap::new(),
        }
    }
}