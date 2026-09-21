//! WASM bindings for the regex_lab_core engine.
//!
//! All public functions are exported to JavaScript via `wasm-bindgen`.
//! Options are passed as JSON strings for simplicity across the FFI boundary.

use crate::engine::{self, RegexOptions};
use crate::error::CoreError;
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a `CoreError` into a `JsValue` with stable `code` and `message` fields.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

/// Parse `RegexOptions` from a JSON string, or return defaults on empty/error.
fn parse_options(options_json: &str) -> RegexOptions {
    if options_json.is_empty() || options_json == "{}" {
        return RegexOptions::default();
    }
    serde_json::from_str::<RegexOptions>(options_json).unwrap_or_default()
}

/// Convert a JSON string to a JsValue by parsing it through JS JSON.parse.
/// This avoids needing serde-wasm-bindgen as a dependency of the core crate.
fn json_to_js(json: &str) -> JsValue {
    js_sys::JSON::parse(json).unwrap_or(JsValue::NULL)
}

// ---------------------------------------------------------------------------
// Exported WASM functions
// ---------------------------------------------------------------------------

/// Test the first match of a regex pattern against input text.
///
/// Returns a JSON object with the `MatchResult` structure.
#[wasm_bindgen]
pub fn wasm_test_match(pattern: &str, options_json: &str, input: &str) -> Result<JsValue, JsValue> {
    let options = parse_options(options_json);
    let result = engine::test_match(pattern, &options, input).map_err(core_err)?;
    let json = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))?;
    Ok(json_to_js(&json))
}

/// Find all non-overlapping matches of a regex pattern.
///
/// Returns a JSON array of `MatchResult` objects.
#[wasm_bindgen]
pub fn wasm_find_all(pattern: &str, options_json: &str, input: &str) -> Result<JsValue, JsValue> {
    let options = parse_options(options_json);
    let results = engine::find_all(pattern, &options, input).map_err(core_err)?;
    let json = serde_json::to_string(&results)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))?;
    Ok(json_to_js(&json))
}

/// Replace matches of a regex pattern with a replacement string.
///
/// Set `replace_all` to `true` to replace every match, `false` for only the first.
#[wasm_bindgen]
pub fn wasm_replace(
    pattern: &str,
    options_json: &str,
    input: &str,
    replacement: &str,
    replace_all: bool,
) -> Result<JsValue, JsValue> {
    let options = parse_options(options_json);
    let result = if replace_all {
        engine::replace_all(pattern, &options, input, replacement)
    } else {
        engine::replace(pattern, &options, input, replacement)
    }
    .map_err(core_err)?;
    let json = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))?;
    Ok(json_to_js(&json))
}

/// Split input text by the regex pattern.
///
/// Returns a JSON array of strings.
#[wasm_bindgen]
pub fn wasm_split(pattern: &str, options_json: &str, input: &str) -> Result<JsValue, JsValue> {
    let options = parse_options(options_json);
    let parts = engine::split(pattern, &options, input).map_err(core_err)?;
    let json = serde_json::to_string(&parts)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))?;
    Ok(json_to_js(&json))
}

/// Validate a regex pattern without executing it.
///
/// Returns `{"valid": true, "error": null}` on success,
/// or `{"valid": false, "error": "..."}` on failure.
#[wasm_bindgen]
pub fn wasm_validate(pattern: &str) -> Result<JsValue, JsValue> {
    match engine::validate(pattern) {
        Ok(()) => {
            let result = serde_json::json!({"valid": true, "error": null});
            let json = serde_json::to_string(&result)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))?;
            Ok(json_to_js(&json))
        }
        Err(e) => {
            let result = serde_json::json!({"valid": false, "error": e.to_string()});
            let json = serde_json::to_string(&result)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))?;
            Ok(json_to_js(&json))
        }
    }
}

/// Generate a code snippet for using this regex pattern in a target language.
///
/// Supported languages: `"javascript"`, `"python"`, `"rust"`, `"go"`.
#[wasm_bindgen]
pub fn wasm_generate_code(pattern: &str, language: &str) -> Result<String, JsValue> {
    // Validate the pattern first (ensures it's a valid regex).
    engine::validate(pattern).map_err(core_err)?;

    let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");

    let code = match language.to_lowercase().as_str() {
        "javascript" | "js" => {
            format!(
                "// JavaScript regex — use with .test(), .match(), .replace(), etc.\n\
                 const regex = /{0}/g;\n\
                 \n\
                 // Test if pattern matches\n\
                 const isMatch = regex.test(input);\n\
                 \n\
                 // Find all matches\n\
                 const matches = [...input.matchAll(regex)];\n\
                 \n\
                 // Replace all matches\n\
                 const replaced = input.replace(regex, 'replacement');",
                pattern
            )
        }
        "python" | "py" => {
            let py_pattern = pattern.replace("\\", "\\\\").replace("'", "\\'");
            format!(
                "# Python regex — use with re.search(), re.findall(), re.sub(), etc.\n\
                 import re\n\
                 \n\
                 pattern = r'{0}'\n\
                 \n\
                 # Test if pattern matches\n\
                 is_match = re.search(pattern, input_text) is not None\n\
                 \n\
                 # Find all matches\n\
                 matches = re.findall(pattern, input_text)\n\
                 \n\
                 # Find all matches with capture groups\n\
                 for m in re.finditer(pattern, input_text):\n\
                 \x20\x20\x20\x20print(m.group(), m.groups())\n\
                 \n\
                 # Replace all matches\n\
                 replaced = re.sub(pattern, 'replacement', input_text)",
                py_pattern
            )
        }
        "rust" | "rs" => {
            let rs_pattern = pattern.replace("\\", "\\\\").replace("\"", "\\\"");
            format!(
                "// Rust regex — add `regex = \"1\"` to Cargo.toml\n\
                 use regex::Regex;\n\
                 \n\
                 let re = Regex::new(r\"{0}\").unwrap();\n\
                 \n\
                 // Test if pattern matches\n\
                 let is_match = re.is_match(input);\n\
                 \n\
                 // Find the first match\n\
                 if let Some(caps) = re.captures(input) {{\n\
                 \x20\x20\x20\x20println!(\"Full match: {{}}\", &caps[0]);\n\
                 }}\n\
                 \n\
                 // Find all matches\n\
                 for caps in re.captures_iter(input) {{\n\
                 \x20\x20\x20\x20println!(\"Match: {{}}\", &caps[0]);\n\
                 }}\n\
                 \n\
                 // Replace all matches\n\
                 let replaced = re.replace_all(input, \"replacement\");",
                rs_pattern
            )
        }
        "go" | "golang" => {
            let go_pattern = pattern
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("`", "\\`");
            format!(
                "// Go regex — use with regexp package\n\
                 import \"regexp\"\n\
                 \n\
                 re := regexp.MustCompile(`{0}`)\n\
                 \n\
                 // Test if pattern matches\n\
                 isMatch := re.MatchString(input)\n\
                 \n\
                 // Find the first match\n\
                 match := re.FindString(input)\n\
                 \n\
                 // Find all matches\n\
                 matches := re.FindAllString(input, -1)\n\
                 \n\
                 // Find submatches (capture groups)\n\
                 submatches := re.FindStringSubmatch(input)\n\
                 \n\
                 // Replace all matches\n\
                 replaced := re.ReplaceAllString(input, \"replacement\")",
                go_pattern
            )
        }
        other => {
            return Err(JsValue::from_str(&format!(
                "Unsupported language: {other}. Supported: javascript, python, rust, go"
            )));
        }
    };

    Ok(code)
}

// NOTE: WASM-specific tests should be run with `wasm-pack test` and require
// wasm-bindgen-test as a dev-dependency. The core engine logic is covered by
// native tests in engine.rs.
