# Changelog

## [0.1.0] - 2026-08-06

### Added
- `regex_lab_core`: Regex engine with test_match, find_all, replace, replace_all, split, validate
- `regex_lab_core`: RegexOptions with full flag support (i, m, s, U, x, u)
- `regex_lab_core`: MatchResult with named groups and capture group extraction
- `regex_lab_core`: ReplaceResult with match count tracking
- `regex_lab_core`: Stable error types (InvalidPattern, PatternTooLong, InputTooLong, ReplacementError)
- `regex_lab_web`: WASM bridge with 6 exported functions + multi-language code generation
- `regex_lab_web`: Full-featured HTML UI with tabs (match, replace, code-gen)
- `regex_lab_web`: Real-time validation with green/red border feedback
- `regex_lab_web`: Inline match highlighting with capture group table
- `regex_lab_web`: Code generation for JavaScript, Python, Rust, and Go
- CI workflow (native test, clippy, WASM check, wasm-pack build)
- Documentation: product spec, AGENTS.md, community files
