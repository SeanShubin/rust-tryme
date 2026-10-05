//! Small, self-contained experiments in Rust language features.
//!
//! Each module isolates one idea and proves it with `#[test]` functions, so
//! `cargo test` is the fastest way to check whether an assumption about the
//! language holds. The console crate runs a few of them for their output.

pub mod dependency_injection;
pub mod errors;
pub mod iterators;
pub mod ownership;
pub mod patterns;
pub mod prototype;
pub mod random;
pub mod serialization;
pub mod traits;
