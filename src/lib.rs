//! # Bio-Logreen Rust SDK
//!
//! The official Rust SDK for the Bio-Logreen Facial Authentication API.

// Declare the modules (files) that are part of our library.
// This tells Rust to look for `client.rs`, `errors.rs`, and `models.rs`.
pub mod client;
pub mod errors;
pub mod models;

// Re-export the most important types so users can easily access them from the top level.
// This allows a developer to write `use biologreen::BioLogreenClient;`
// instead of the longer `use biologreen::client::BioLogreenClient;`.
pub use client::BioLogreenClient;
pub use errors::{Error, Result};
pub use models::FaceAuthResponse;

