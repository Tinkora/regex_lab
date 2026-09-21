mod engine;
mod error;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use engine::{
    MatchResult, RegexOptions, ReplaceResult, find_all, replace, replace_all, split, test_match,
    validate,
};
pub use error::CoreError;
