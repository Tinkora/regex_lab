# Repository Guide for AI Agents

## Project Overview

regex_lab is a browser-native regular expression testing laboratory. Real-time matching, capture group extraction, replace preview, and multi-language code generation — all powered by Rust/WASM running entirely in the browser.

## Architecture

```
regex_lab/
├── crates/
│   ├── regex_lab_core/       # Regex engine, match/replace/split logic, error types
│   └── regex_lab_web/        # WASM bridge + HTML editor UI
├── docs/                      # Product specification
├── skills/                    # Agent Skill definitions (MCP tools)
└── index.html                 # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/regex_lab_core/src/engine.rs` | RegexOptions, MatchResult, ReplaceResult, test_match, find_all, replace, replace_all, split, validate |
| `crates/regex_lab_core/src/error.rs` | CoreError: InvalidPattern, PatternTooLong, InputTooLong, ReplacementError |
| `crates/regex_lab_core/src/wasm.rs` | WASM bindings: wasm_test_match, wasm_find_all, wasm_replace, wasm_split, wasm_validate, wasm_generate_code |
| `crates/regex_lab_web/src/lib.rs` | WASM entry point, re-exports core WASM functions |
| `crates/regex_lab_web/static/index.html` | Full-featured HTML UI with tabs for match/replace/code-gen |
| `skills/regex_lab.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |
| `docs/product_spec.zh-CN.md` | Product specification (Chinese) |

## Build & Test Commands

```bash
# Run all native tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p regex_lab_core --target wasm32-unknown-unknown
cargo check -p regex_lab_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/regex_lab_web

# Serve locally
cp crates/regex_lab_web/pkg/* crates/regex_lab_web/static/pkg/
cd crates/regex_lab_web/static && python3 -m http.server 8080
```

## Design Principles

1. **Browser-first**: All regex processing happens in-browser via WASM — no server round-trips
2. **Real-time feedback**: Pattern validation and match results update on every keystroke
3. **Zero data exfiltration**: Test strings never leave the browser
4. **Multi-language**: Generate equivalent regex code snippets for JS, Python, Rust, and Go
5. **Comprehensive flag support**: Full coverage of common regex flags (i, m, s, U, x, u)

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `INVALID_PATTERN` | Regular expression pattern is syntactically invalid |
| `PATTERN_TOO_LONG` | Pattern exceeds maximum allowed length |
| `INPUT_TOO_LONG` | Test input exceeds maximum allowed length |
| `REPLACEMENT_ERROR` | Replacement string is invalid for the pattern |

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
