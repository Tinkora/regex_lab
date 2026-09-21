//! regex_lab_web — WASM entry point for the regex_lab web application.
//!
//! This crate re-exports all WASM functions from `regex_lab_core::wasm` so that
//! `wasm-pack` picks them up from the cdylib crate.

#[cfg(target_arch = "wasm32")]
pub use regex_lab_core::wasm::*;
