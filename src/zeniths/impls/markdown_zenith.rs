// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::{Result, ZenithError};
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct MarkdownZenith;

const SUPPORTED_LANGUAGES: &[&str] = &[
    "rust",
    "python",
    "javascript",
    "typescript",
    "js",
    "ts",
    "go",
    "java",
    "c",
    "cpp",
    "csharp",
    "ruby",
    "php",
    "swift",
    "kotlin",
    "sql",
    "html",
    "css",
    "json",
    "yaml",
    "bash",
    "shell",
    "powershell",
];

// Compile regex with proper error handling
macro_rules! try_lazy_regex {
    ($name:ident, $pattern:expr) => {
        static $name: ::once_cell::sync::Lazy<std::result::Result<regex::Regex, regex::Error>> =
            ::once_cell::sync::Lazy::new(|| regex::Regex::new($pattern));
    };
}

try_lazy_regex!(INLINE_CODE_PATTERN, r#"`([^`]+)`"#);
try_lazy_regex!(TASK_LIST_PATTERN, r#"(?m)^(\s*)(-\s+)\[(\s*)\]\s+(.+)$"#);
try_lazy_regex!(STRIKETHROUGH_PATTERN, r"~~([^~]+)~~");
try_lazy_regex!(LINK_PATTERN, r"\[([^\]]+)\]\(([^)]+)\)");
try_lazy_regex!(BOLD_PATTERN, r"\*\*([^*]+)\*\*");
try_lazy_regex!(ITALIC_PATTERN, r"\*([^*]+)\*");
try_lazy_regex!(BOLD_ITALIC_PATTERN, r"\*\*\*([^*]+)\*\*\*");
try_lazy_regex!(
    HORIZONTAL_RULE_PATTERN,
    r"(?m)^(\s*)(-{3,}|\*{3,}|_{3,})\s*$"
);
try_lazy_regex!(MULTI_LINE_CODE_PATTERN, r"(?s)```(\w+)\s*\n(.+?)\n```");
try_lazy_regex!(SINGLE_LINE_CODE_PATTERN, r"(?s)```(\w+)\s+([^\n]+?)\s*```");

/// Safely get a regex from a Lazy<Result<Regex, Error>>, converting errors to ZenithError
macro_rules! get_regex {
    ($name:ident) => {
        match &$name {
            lazy_regex => {
                let result = lazy_regex.as_ref();
                match result {
                    Ok(regex) => regex.clone(),
                    Err(e) => {
                        return Err(ZenithError::Config(format!(
                            "Failed to compile regex: {}",
                            e
                        )));
                    }
                }
            }
        }
    };
}

#[async_trait]
impl Zenith for MarkdownZenith {
    fn name(&self) -> &str {
        "markdown"
    }

    fn extensions(&self) -> &[&str] {
        &["md"]
    }

    fn priority(&self) -> i32 {
        100
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        let preprocessed = preprocess_extremely_compressed(content)?;
        let with_inline_code_formatted = format_inline_code(&preprocessed)?;
        let with_task_lists = format_task_lists(&with_inline_code_formatted)?;
        let with_strikethrough = format_strikethrough(&with_task_lists)?;
        let with_links = format_links_and_images(&with_strikethrough)?;
        let with_emphasis = format_emphasis(&with_links)?;
        let with_horizontal_rules = format_horizontal_rules(&with_emphasis)?;
        let with_rust_formatted = format_rust_code_blocks(&with_horizontal_rules)?;
        let formatter = StdioFormatter {
            tool_name: "prettier",
            args: vec![
                "--stdin-filepath".into(),
                "--parser".into(),
                "markdown".into(),
            ],
            timeout_seconds: None,
        };
        formatter
            .format_with_stdio_no_path(with_rust_formatted.as_bytes(), path, None)
            .await
    }
}

fn preprocess_extremely_compressed(content: &[u8]) -> Result<String> {
    let text = String::from_utf8_lossy(content);
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0usize;

    let mut stall_count = 0usize;
    const MAX_STALL: usize = 1000000;

    while i < chars.len() {
        stall_count += 1;
        if stall_count > MAX_STALL {
            eprintln!(
                "[WARN] Detected potential infinite loop in preprocessing at position {}",
                i
            );
            break;
        }

        if is_header_start(&chars, i) {
            let header_result = parse_header(&chars, i)?;
            result.push_str(&header_result.text);
            result.push('\n');
            i = header_result.next_pos;
        } else if is_table_start(&chars, i) {
            let table_result = parse_table(&chars, i)?;
            result.push_str(&table_result.text);
            result.push('\n');
            i = table_result.next_pos;
        } else if is_blockquote_start(&chars, i) {
            let quote_result = parse_blockquote(&chars, i)?;
            result.push_str(&quote_result.text);
            result.push('\n');
            i = quote_result.next_pos;
        } else if is_unordered_list_start(&chars, i) {
            let list_result = parse_list(&chars, i)?;
            result.push_str(&list_result.text);
            result.push('\n');
            i = list_result.next_pos;
        } else if is_ordered_list_start(&chars, i) {
            let list_result = parse_ordered_list(&chars, i)?;
            result.push_str(&list_result.text);
            result.push('\n');
            i = list_result.next_pos;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    Ok(result.trim().to_string())
}

fn is_header_start(chars: &[char], i: usize) -> bool {
    chars[i] == '#' && (i == 0 || chars[i - 1] == ' ' || chars[i - 1] == '\n')
}

fn is_table_start(chars: &[char], i: usize) -> bool {
    chars[i] == '|' && (i == 0 || chars[i - 1] == ' ' || chars[i - 1] == '\n')
}

fn is_blockquote_start(chars: &[char], i: usize) -> bool {
    chars[i] == '>' && (i == 0 || chars[i - 1] == ' ')
}

fn is_unordered_list_start(chars: &[char], i: usize) -> bool {
    (chars[i] == '-' || chars[i] == '*' || chars[i] == '+') && (i == 0 || chars[i - 1] == ' ')
}

fn is_ordered_list_start(chars: &[char], i: usize) -> bool {
    if !chars[i].is_ascii_digit() {
        return false;
    }
    let mut j = i;
    while j < chars.len() && chars[j].is_ascii_digit() {
        j += 1;
    }
    if j < chars.len() && (chars[j] == '.' || chars[j] == ')') {
        return true;
    }
    false
}

struct ParseResult {
    text: String,
    next_pos: usize,
}

fn parse_header(chars: &[char], mut i: usize) -> Result<ParseResult> {
    let header_start = i;
    while i < chars.len() && chars[i] == '#' {
        i += 1;
    }
    while i < chars.len() && chars[i] == ' ' {
        i += 1;
    }
    let title_start = i;

    let mut next_pos = i;
    while next_pos < chars.len() && !is_header_start(chars, next_pos) {
        if chars[next_pos] == '|' && next_pos > 0 && chars[next_pos - 1] != ' ' {
            next_pos += 1;
            continue;
        }
        if chars[next_pos] == '>' && (next_pos == 0 || chars[next_pos - 1] == ' ') {
            break;
        }
        if (chars[next_pos] == '-' || chars[next_pos] == '*' || chars[next_pos] == '+')
            && (next_pos == 0 || chars[next_pos - 1] == ' ')
        {
            break;
        }
        next_pos += 1;
        if next_pos >= chars.len() {
            break;
        }
    }

    let title_text: String = chars[title_start..next_pos].iter().collect();
    let header_pattern: String = chars[header_start..title_start].iter().collect();

    // Validate title text to prevent potential injection issues
    let validated_title = validate_title_text(&title_text)?;

    Ok(ParseResult {
        text: format!("{}{}", header_pattern, validated_title),
        next_pos,
    })
}

/// Validate title text to prevent path traversal and other injection attacks
fn validate_title_text(text: &str) -> Result<String> {
    // Check for null bytes and control characters
    for ch in text.chars() {
        if ch == '\0' || (ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t') {
            return Err(ZenithError::Config(
                "Invalid characters in title text".to_string(),
            ));
        }
    }

    // Check for potential path traversal attempts
    if text.contains("..") || text.contains('\0') {
        return Err(ZenithError::PathTraversal(PathBuf::from(text.to_string())));
    }

    Ok(text.trim().to_string())
}

fn parse_table(chars: &[char], i: usize) -> Result<ParseResult> {
    let table_start = i;
    let mut table_end = i;
    let mut has_content = false;

    while table_end < chars.len() {
        if is_header_start(chars, table_end) {
            break;
        }
        if is_blockquote_start(chars, table_end) {
            break;
        }
        if is_unordered_list_start(chars, table_end) {
            break;
        }
        if is_ordered_list_start(chars, table_end) {
            break;
        }
        if chars[table_end] == '|' {
            has_content = true;
        }
        table_end += 1;
        if table_end >= chars.len() {
            break;
        }
    }

    if !has_content {
        return Ok(ParseResult {
            text: chars[table_start..table_end].iter().collect(),
            next_pos: table_end,
        });
    }

    let table_text: String = chars[table_start..table_end].iter().collect();
    let mut result = String::new();
    process_table(&table_text, &mut result)?;

    Ok(ParseResult {
        text: result.trim().to_string(),
        next_pos: table_end,
    })
}

fn parse_blockquote(chars: &[char], i: usize) -> Result<ParseResult> {
    let quote_start = i;
    let mut quote_end = i;
    while quote_end < chars.len() && !is_header_start(chars, quote_end) {
        if is_unordered_list_start(chars, quote_end) || is_ordered_list_start(chars, quote_end) {
            break;
        }
        quote_end += 1;
        if quote_end >= chars.len() {
            break;
        }
    }

    let quote_text: String = chars[quote_start..quote_end].iter().collect();

    Ok(ParseResult {
        text: quote_text,
        next_pos: quote_end,
    })
}

fn parse_list(chars: &[char], i: usize) -> Result<ParseResult> {
    let list_start = i;
    let mut list_end = i;

    while list_end < chars.len() && chars[list_end] != '\n' {
        if is_header_start(chars, list_end) {
            break;
        }
        if is_blockquote_start(chars, list_end) {
            break;
        }
        if is_table_start(chars, list_end) {
            break;
        }
        list_end += 1;
        if list_end >= chars.len() {
            break;
        }
    }

    let list_text: String = chars[list_start..list_end].iter().collect();
    let items: Vec<&str> = list_text.split(" - ").collect();

    let mut result = String::new();
    if items.len() > 1 {
        for item in items {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('+')
                {
                    result.push_str(trimmed);
                    result.push('\n');
                } else if !trimmed.is_empty() {
                    result.push_str("- ");
                    result.push_str(trimmed);
                    result.push('\n');
                }
            }
        }
    } else if list_text.trim().starts_with('-')
        || list_text.trim().starts_with('*')
        || list_text.trim().starts_with('+')
    {
        result.push_str(list_text.trim());
    } else {
        result.push_str("- ");
        result.push_str(list_text.trim());
    }

    Ok(ParseResult {
        text: result.trim().to_string(),
        next_pos: list_end,
    })
}

fn parse_ordered_list(chars: &[char], i: usize) -> Result<ParseResult> {
    let list_start = i;
    let mut list_end = i;

    while list_end < chars.len() && chars[list_end] != '\n' {
        if is_header_start(chars, list_end) {
            break;
        }
        if is_blockquote_start(chars, list_end) {
            break;
        }
        if is_table_start(chars, list_end) {
            break;
        }
        list_end += 1;
        if list_end >= chars.len() {
            break;
        }
    }

    let list_text: String = chars[list_start..list_end].iter().collect();

    Ok(ParseResult {
        text: list_text,
        next_pos: list_end,
    })
}

fn format_inline_code(text: &str) -> Result<String> {
    let regex = get_regex!(INLINE_CODE_PATTERN);

    let mut result = text.to_string();

    let replacements: Vec<(String, String)> = regex
        .captures_iter(&result)
        .filter_map(|cap| {
            let full_match = cap.get(0)?.as_str().to_string();
            let code_content = cap.get(1)?.as_str().to_string();
            let lang = detect_inline_language(&code_content);
            if lang == "rust" {
                if let Ok(formatted) = format_with_rustfmt(&code_content) {
                    let cleaned = clean_inline_code(&formatted);
                    return Some((full_match, format!("`{}`", cleaned)));
                }
            }
            None
        })
        .collect();

    for (original, replacement) in replacements.iter().rev() {
        if let Some(pos) = result.rfind(original) {
            let before = &result[..pos];
            let after = &result[pos + original.len()..];
            result = format!("{}{}{}", before, replacement, after);
        }
    }

    Ok(result)
}

fn detect_inline_language(code: &str) -> &'static str {
    let trimmed = code.trim();
    if trimmed.starts_with("fn ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("impl ")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("enum ")
        || trimmed.starts_with("trait ")
    {
        return "rust";
    }
    if trimmed.starts_with("def ")
        || trimmed.starts_with("class ")
        || trimmed.starts_with("import ")
        || trimmed.starts_with("from ")
    {
        return "python";
    }
    if trimmed.starts_with("function ")
        || trimmed.starts_with("const ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("var ")
        || trimmed.contains("=>")
    {
        return "javascript";
    }
    ""
}

fn clean_inline_code(formatted: &str) -> String {
    formatted
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

fn format_task_lists(text: &str) -> Result<String> {
    let regex = get_regex!(TASK_LIST_PATTERN);
    Ok(regex.replace_all(text, "${1}${2}[ ] ${4}").to_string())
}

fn format_strikethrough(text: &str) -> Result<String> {
    let regex = get_regex!(STRIKETHROUGH_PATTERN);
    Ok(regex.replace_all(text, "~~$1~~").to_string())
}

fn format_links_and_images(text: &str) -> Result<String> {
    let regex = get_regex!(LINK_PATTERN);
    Ok(regex.replace_all(text, "[$1]($2)").to_string())
}

fn format_emphasis(text: &str) -> Result<String> {
    let regex_bold_italic = get_regex!(BOLD_ITALIC_PATTERN);
    let regex_bold = get_regex!(BOLD_PATTERN);
    let regex_italic = get_regex!(ITALIC_PATTERN);

    let mut result = text.to_string();
    result = regex_bold_italic
        .replace_all(&result, "***$1***")
        .to_string();
    result = regex_bold.replace_all(&result, "**$1**").to_string();
    result = regex_italic.replace_all(&result, "*$1*").to_string();

    Ok(result)
}

fn format_horizontal_rules(text: &str) -> Result<String> {
    let regex = get_regex!(HORIZONTAL_RULE_PATTERN);
    Ok(regex.replace_all(text, "---").to_string())
}

fn format_rust_code_blocks(content: &str) -> Result<String> {
    let multi_regex = get_regex!(MULTI_LINE_CODE_PATTERN);
    let single_regex = get_regex!(SINGLE_LINE_CODE_PATTERN);

    let mut result = content.to_string();

    let replacements: Vec<(String, String, String)> = multi_regex
        .captures_iter(&result)
        .filter_map(|cap| {
            let lang = cap.get(1)?.as_str();
            if !SUPPORTED_LANGUAGES.contains(&lang) {
                return None;
            }
            let full_match = cap.get(0)?.as_str().to_string();
            let code_content = cap.get(2)?.as_str().to_string();
            let formatted = if lang == "rust" {
                format_with_rustfmt(&code_content).ok()?
            } else {
                code_content
            };
            Some((full_match, lang.to_string(), formatted))
        })
        .collect();

    for (original, lang, formatted) in replacements.iter().rev() {
        let replacement = format!("```{}\n{}\n```", lang, formatted);
        if let Some(pos) = result.rfind(original) {
            let before = &result[..pos];
            let after = &result[pos + original.len()..];
            result = format!("{}{}{}", before, replacement, after);
        }
    }

    let single_replacements: Vec<(String, String, String)> = single_regex
        .captures_iter(&result)
        .filter_map(|cap| {
            let lang = cap.get(1)?.as_str();
            if !SUPPORTED_LANGUAGES.contains(&lang) {
                return None;
            }
            let full_match = cap.get(0)?.as_str().to_string();
            let code_content = cap.get(2)?.as_str().to_string();
            let formatted = if lang == "rust" {
                format_with_rustfmt(&code_content).ok()?
            } else {
                code_content
            };
            let cleaned = clean_inline_code(&formatted);
            Some((full_match, lang.to_string(), cleaned))
        })
        .collect();

    for (original, lang, formatted) in single_replacements.iter().rev() {
        let replacement = format!("```{}\n{}\n```", lang, formatted);
        if let Some(pos) = result.rfind(original) {
            let before = &result[..pos];
            let after = &result[pos + original.len()..];
            result = format!("{}{}{}", before, replacement, after);
        }
    }

    Ok(result)
}

fn format_with_rustfmt(code: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            ZenithError::Io(std::io::Error::other(format!(
                "Failed to spawn rustfmt: {}",
                e
            )))
        })?;

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(code.as_bytes()).map_err(|e| {
            ZenithError::Io(std::io::Error::other(format!(
                "Failed to write to rustfmt stdin: {}",
                e
            )))
        })?;
    }

    let output = child.wait_with_output().map_err(|e| {
        ZenithError::Io(std::io::Error::other(format!(
            "Failed to read rustfmt output: {}",
            e
        )))
    })?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(ZenithError::Utf8Conversion)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(ZenithError::ZenithFailed {
            name: "rustfmt".to_string(),
            reason: error_msg.to_string(),
        })
    }
}

fn is_separator_cell(cell: &str) -> bool {
    let trimmed = cell.trim();
    if trimmed.is_empty() {
        return true;
    }
    let sep_chars: Vec<char> = trimmed.chars().collect();
    if sep_chars.is_empty() {
        return false;
    }
    let sep_count = sep_chars.iter().filter(|&&c| c == '-' || c == ':').count();
    let total = sep_chars.len();
    sep_count == total && total >= 3
}

fn process_table(table_text: &str, result: &mut String) -> Result<()> {
    let raw_cells: Vec<&str> = table_text.split('|').collect();
    let mut cells: Vec<String> = raw_cells.iter().map(|&s| s.trim().to_string()).collect();

    if cells.is_empty() {
        return Ok(());
    }

    while cells.last().is_some_and(|s| s.is_empty()) {
        cells.pop();
    }

    if cells.is_empty() {
        return Ok(());
    }

    let first_is_empty = cells.first().is_some_and(|s| s.is_empty());
    let start_idx = if first_is_empty { 1 } else { 0 };
    let data_cells: Vec<String> = cells[start_idx..].to_vec();

    if data_cells.len() < 2 {
        return Ok(());
    }

    let mut header_end = 0;
    for (idx, cell) in data_cells.iter().enumerate() {
        if is_separator_cell(cell) {
            break;
        }
        header_end = idx + 1;
    }

    if header_end < 1 {
        return Ok(());
    }

    let mut separator_end = header_end;
    let mut found_non_separator = false;
    for (idx, cell) in data_cells.iter().enumerate().skip(header_end) {
        if is_separator_cell(cell) {
            if !found_non_separator {
                separator_end = idx + 1;
            }
        } else {
            found_non_separator = true;
        }
    }

    let num_cols = header_end;

    result.push('|');
    for (idx, cell) in data_cells[..header_end].iter().enumerate() {
        result.push_str(cell);
        if idx < header_end - 1 {
            result.push('|');
        }
    }
    result.push('|');
    result.push('\n');

    if separator_end > header_end {
        result.push('|');
        for (idx, cell) in data_cells[header_end..separator_end].iter().enumerate() {
            result.push_str(cell);
            if idx < separator_end - header_end - 1 {
                result.push('|');
            }
        }
        result.push('|');
        result.push('\n');
    } else {
        for _col in 0..num_cols {
            result.push('|');
            result.push_str("---");
        }
        result.push('|');
        result.push('\n');
    }

    let data_start = separator_end;
    let remaining_cells: Vec<String> = data_cells[data_start..]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();
    let total_data_cells = remaining_cells.len();
    let full_rows = total_data_cells / num_cols;

    let mut cell_idx = 0;
    for _row in 0..full_rows {
        result.push('|');
        for col in 0..num_cols {
            result.push_str(&remaining_cells[cell_idx]);
            cell_idx += 1;
            if col < num_cols - 1 {
                result.push('|');
            }
        }
        result.push('|');
        result.push('\n');
    }

    Ok(())
}
