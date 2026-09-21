use crate::error::{CoreError, MAX_INPUT_LENGTH, MAX_PATTERN_LENGTH};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Options controlling regex matching behavior.
///
/// These map to the standard regex flags supported by the `regex` crate.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegexOptions {
    /// Case-insensitive matching (flag `i`).
    #[serde(default)]
    pub case_insensitive: bool,

    /// Multi-line mode: `^` and `$` match line boundaries (flag `m`).
    #[serde(default)]
    pub multi_line: bool,

    /// Dot matches newline characters (flag `s`).
    #[serde(default)]
    pub dot_matches_new_line: bool,

    /// Swap greediness: `*` and `+` become non-greedy, `*?` and `+?` become greedy (flag `U`).
    #[serde(default)]
    pub swap_greed: bool,

    /// Ignore whitespace and `#` comments in the pattern (flag `x`).
    #[serde(default)]
    pub ignore_whitespace: bool,

    /// Enable Unicode-aware matching (flag `u`). Enabled by default.
    #[serde(default = "default_unicode")]
    pub unicode: bool,

    /// Enable octal escapes in the pattern (rarely used).
    #[serde(default)]
    pub octal: bool,
}

const fn default_unicode() -> bool {
    true
}

impl Default for RegexOptions {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            ignore_whitespace: false,
            unicode: true,
            octal: false,
        }
    }
}

/// The result of a single regex match.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchResult {
    /// Whether the pattern matched.
    pub is_match: bool,

    /// Byte offset of the start of the match.
    pub match_start: usize,

    /// Byte offset of the end of the match.
    pub match_end: usize,

    /// The matched text.
    pub matched_text: String,

    /// Numbered capture groups (index 0 is the full match).
    pub groups: Vec<Option<String>>,

    /// Named capture groups, keyed by name.
    #[serde(default)]
    pub named_groups: BTreeMap<String, Option<String>>,
}

/// The result of a regex replace operation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceResult {
    /// The text after replacement.
    pub replaced: String,

    /// How many matches were replaced.
    pub match_count: usize,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Build a `Regex` from a pattern and options, checking length limits first.
fn build_regex(pattern: &str, options: &RegexOptions) -> Result<Regex, CoreError> {
    if pattern.len() > MAX_PATTERN_LENGTH {
        return Err(CoreError::PatternTooLong(pattern.len()));
    }

    RegexBuilder::new(pattern)
        .case_insensitive(options.case_insensitive)
        .multi_line(options.multi_line)
        .dot_matches_new_line(options.dot_matches_new_line)
        .swap_greed(options.swap_greed)
        .ignore_whitespace(options.ignore_whitespace)
        .unicode(options.unicode)
        .octal(options.octal)
        .build()
        .map_err(|e| CoreError::InvalidPattern(e.to_string()))
}

/// Check input length against the limit.
fn check_input_length(input: &str) -> Result<(), CoreError> {
    if input.len() > MAX_INPUT_LENGTH {
        return Err(CoreError::InputTooLong(input.len(), MAX_INPUT_LENGTH));
    }
    Ok(())
}

/// Extract match information from a `regex::Captures` into a `MatchResult`.
fn captures_to_match_result(captures: &regex::Captures<'_>, re: &Regex) -> MatchResult {
    let full = captures.get(0);
    let mut groups: Vec<Option<String>> = Vec::new();

    for i in 0..captures.len() {
        groups.push(captures.get(i).map(|m| m.as_str().to_string()));
    }

    let mut named_groups: BTreeMap<String, Option<String>> = BTreeMap::new();
    for name in re.capture_names().flatten() {
        named_groups.insert(
            name.to_string(),
            captures.name(name).map(|m| m.as_str().to_string()),
        );
    }

    MatchResult {
        is_match: true,
        match_start: full.map(|m| m.start()).unwrap_or(0),
        match_end: full.map(|m| m.end()).unwrap_or(0),
        matched_text: full.map(|m| m.as_str().to_string()).unwrap_or_default(),
        groups,
        named_groups,
    }
}

/// Build an empty "no match" result.
fn no_match_result() -> MatchResult {
    MatchResult {
        is_match: false,
        match_start: 0,
        match_end: 0,
        matched_text: String::new(),
        groups: Vec::new(),
        named_groups: BTreeMap::new(),
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Test a regex pattern against input text and return the first match.
///
/// Returns `MatchResult` with `is_match: false` if no match is found.
pub fn test_match(
    pattern: &str,
    options: &RegexOptions,
    input: &str,
) -> Result<MatchResult, CoreError> {
    check_input_length(input)?;
    let re = build_regex(pattern, options)?;

    match re.captures(input) {
        Some(caps) => Ok(captures_to_match_result(&caps, &re)),
        None => Ok(no_match_result()),
    }
}

/// Find all non-overlapping matches of a regex pattern in input text.
pub fn find_all(
    pattern: &str,
    options: &RegexOptions,
    input: &str,
) -> Result<Vec<MatchResult>, CoreError> {
    check_input_length(input)?;
    let re = build_regex(pattern, options)?;

    let results: Vec<MatchResult> = re
        .captures_iter(input)
        .map(|caps| captures_to_match_result(&caps, &re))
        .collect();

    Ok(results)
}

/// Replace the first match of a regex pattern with a replacement string.
///
/// The replacement string may reference capture groups using `$1`, `$name`, etc.
pub fn replace(
    pattern: &str,
    options: &RegexOptions,
    input: &str,
    replacement: &str,
) -> Result<ReplaceResult, CoreError> {
    check_input_length(input)?;
    let re = build_regex(pattern, options)?;

    // Check if there is at least one match.
    let match_count = if re.is_match(input) { 1 } else { 0 };

    let replaced = re
        .replacen(input, 1, replacement)
        .to_string();

    Ok(ReplaceResult {
        replaced,
        match_count,
    })
}

/// Replace all matches of a regex pattern with a replacement string.
///
/// The replacement string may reference capture groups using `$1`, `$name`, etc.
pub fn replace_all(
    pattern: &str,
    options: &RegexOptions,
    input: &str,
    replacement: &str,
) -> Result<ReplaceResult, CoreError> {
    check_input_length(input)?;
    let re = build_regex(pattern, options)?;

    let match_count = re.find_iter(input).count();

    let replaced = re.replace_all(input, replacement).to_string();

    Ok(ReplaceResult {
        replaced,
        match_count,
    })
}

/// Split input text by the regex pattern.
pub fn split(
    pattern: &str,
    options: &RegexOptions,
    input: &str,
) -> Result<Vec<String>, CoreError> {
    check_input_length(input)?;
    let re = build_regex(pattern, options)?;

    let parts: Vec<String> = re.split(input).map(|s| s.to_string()).collect();

    Ok(parts)
}

/// Validate a regex pattern without executing it.
///
/// Returns `Ok(())` if the pattern compiles successfully.
pub fn validate(pattern: &str) -> Result<(), CoreError> {
    if pattern.len() > MAX_PATTERN_LENGTH {
        return Err(CoreError::PatternTooLong(pattern.len()));
    }

    Regex::new(pattern).map_err(|e| CoreError::InvalidPattern(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_opts() -> RegexOptions {
        RegexOptions::default()
    }

    #[test]
    fn default_options_enable_unicode() {
        assert!(default_opts().unicode);
    }

    // ---- test_match ----

    #[test]
    fn test_match_simple() {
        let r = test_match(r"\d+", &default_opts(), "abc 123 def").unwrap();
        assert!(r.is_match);
        assert_eq!(r.matched_text, "123");
        assert_eq!(r.match_start, 4);
        assert_eq!(r.match_end, 7);
    }

    #[test]
    fn test_match_no_match() {
        let r = test_match(r"\d+", &default_opts(), "abc def").unwrap();
        assert!(!r.is_match);
        assert!(r.matched_text.is_empty());
    }

    #[test]
    fn test_match_with_groups() {
        let r = test_match(r"(\w+)@([\w.]+)", &default_opts(), "hello world user@example.com test")
            .unwrap();
        assert!(r.is_match);
        assert_eq!(r.groups[0].as_deref(), Some("user@example.com"));
        assert_eq!(r.groups[1].as_deref(), Some("user"));
        assert_eq!(r.groups[2].as_deref(), Some("example.com"));
    }

    #[test]
    fn test_match_with_named_groups() {
        let r = test_match(
            r"(?P<name>\w+)@(?P<domain>\w+\.\w+)",
            &default_opts(),
            "hello user@example.com test",
        )
        .unwrap();
        assert!(r.is_match);
        assert_eq!(
            r.named_groups.get("name").unwrap().as_deref(),
            Some("user")
        );
        assert_eq!(
            r.named_groups.get("domain").unwrap().as_deref(),
            Some("example.com")
        );
    }

    #[test]
    fn test_match_case_insensitive() {
        let mut opts = default_opts();
        opts.case_insensitive = true;
        let r = test_match(r"hello", &opts, "HELLO world").unwrap();
        assert!(r.is_match);
    }

    // ---- find_all ----

    #[test]
    fn test_find_all_multiple() {
        let results = find_all(r"\d+", &default_opts(), "a1 b22 c333").unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].matched_text, "1");
        assert_eq!(results[1].matched_text, "22");
        assert_eq!(results[2].matched_text, "333");
    }

    #[test]
    fn test_find_all_none() {
        let results = find_all(r"\d+", &default_opts(), "abc def").unwrap();
        assert!(results.is_empty());
    }

    // ---- replace ----

    #[test]
    fn test_replace_first() {
        let r = replace(r"cat", &default_opts(), "cat and cat", "dog").unwrap();
        assert_eq!(r.replaced, "dog and cat");
        assert_eq!(r.match_count, 1);
    }

    #[test]
    fn test_replace_no_match() {
        let r = replace(r"cat", &default_opts(), "dog and dog", "bird").unwrap();
        assert_eq!(r.replaced, "dog and dog");
        assert_eq!(r.match_count, 0);
    }

    // ---- replace_all ----

    #[test]
    fn test_replace_all_multiple() {
        let r = replace_all(r"cat", &default_opts(), "cat and cat", "dog").unwrap();
        assert_eq!(r.replaced, "dog and dog");
        assert_eq!(r.match_count, 2);
    }

    #[test]
    fn test_replace_all_with_capture_refs() {
        let r = replace_all(
            r"(\w+),\s*(\w+)",
            &default_opts(),
            "Doe, John; Smith, Jane",
            "$2 $1",
        )
        .unwrap();
        assert_eq!(r.replaced, "John Doe; Jane Smith");
        assert_eq!(r.match_count, 2);
    }

    // ---- split ----

    #[test]
    fn test_split_simple() {
        let parts = split(r",\s*", &default_opts(), "a, b, c").unwrap();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_no_match() {
        let parts = split(r"[,]", &default_opts(), "abc").unwrap();
        assert_eq!(parts, vec!["abc"]);
    }

    // ---- validate ----

    #[test]
    fn test_validate_valid() {
        assert!(validate(r"\d+").is_ok());
        assert!(validate(r"(?P<x>hello)").is_ok());
    }

    #[test]
    fn test_validate_invalid() {
        let err = validate(r"[unclosed").unwrap_err();
        assert!(matches!(err, CoreError::InvalidPattern(_)));
    }

    // ---- error codes ----

    #[test]
    fn test_error_codes() {
        assert_eq!(
            CoreError::InvalidPattern("bad".into()).code(),
            "INVALID_PATTERN"
        );
        assert_eq!(
            CoreError::PatternTooLong(100000).code(),
            "PATTERN_TOO_LONG"
        );
        assert_eq!(
            CoreError::InputTooLong(20000000, MAX_INPUT_LENGTH).code(),
            "INPUT_TOO_LONG"
        );
        assert_eq!(
            CoreError::ReplacementError("oops".into()).code(),
            "REPLACEMENT_ERROR"
        );
    }

    // ---- flag combinations ----

    #[test]
    fn test_multi_line_flag() {
        let mut opts = default_opts();
        opts.multi_line = true;
        let results = find_all(r"^\w+", &opts, "hello\nworld").unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].matched_text, "hello");
        assert_eq!(results[1].matched_text, "world");
    }

    #[test]
    fn test_dot_matches_newline_flag() {
        let mut opts = default_opts();
        opts.dot_matches_new_line = true;
        let r = test_match(r"a.+z", &opts, "a\nz").unwrap();
        assert!(r.is_match);
    }

    #[test]
    fn test_swap_greed_flag() {
        let mut opts = default_opts();
        opts.swap_greed = true;
        // With swap_greed, * is non-greedy (like *?)
        let r = test_match(r"<.*>", &opts, "<a> <b>").unwrap();
        assert_eq!(r.matched_text, "<a>");
    }

    #[test]
    fn test_pattern_too_long() {
        let long = "a".repeat(MAX_PATTERN_LENGTH + 1);
        let err = test_match(&long, &default_opts(), "test").unwrap_err();
        assert!(matches!(err, CoreError::PatternTooLong(_)));
    }
}
