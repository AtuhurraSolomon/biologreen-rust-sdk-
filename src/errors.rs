use reqwest::StatusCode;
use thiserror::Error;

/// The main error type for the BioLogreen SDK.
/// This enum represents all possible failures that can occur.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred during the network request itself.
    /// This could be due to a DNS issue, the server being down, or other network problems.
    /// This wraps the original error from the `reqwest` library.
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    /// The BioLogreen API returned a non-successful status code (e.g., 400, 401, 404).
    /// This is the direct equivalent of the `BioLogreenApiException` in the C# SDK.
    #[error("API error: {message} (Status: {status})")]
    Api {
        status: StatusCode,
        message: String,
    },
}

/// A convenient type alias for `Result<T, Error>`.
/// This simplifies function signatures throughout our SDK, so instead of writing
/// `Result<FaceAuthResponse, biologreen::errors::Error>`, we can just write `Result<FaceAuthResponse>`.
pub type Result<T> = std::result::Result<T, Error>;

