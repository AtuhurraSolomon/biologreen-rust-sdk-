use crate::errors::{Error, Result};
use crate::models::{FaceAuthResponse, LoginRequest, SignupRequest};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

// The public client for interacting with the BioLogreen API.
pub struct BioLogreenClient {
    client: Client,
    base_url: String,
}

impl BioLogreenClient {
    /// Creates a new instance of the BioLogreenClient.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your secret API key for authentication.
    /// * `base_url` - Optional base URL for the BioLogreen API. Defaults to the production URL if not provided.
    ///
    /// # Returns
    ///
    /// A new `BioLogreenClient` instance.
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("X-API-KEY", HeaderValue::from_str(&api_key).unwrap());
        headers.insert(USER_AGENT, HeaderValue::from_static("BioLogreen-Rust-SDK/0.1.0"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        let base_url = base_url.unwrap_or_else(|| "https://api.biologreen.com/v1".to_string());

        BioLogreenClient { client, base_url }
    }

    /// Asynchronously registers a new user by their face.
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - The raw byte array of the user's face image (e.g., from a JPEG or PNG file).
    /// * `custom_fields` - Optional custom data to store with the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FaceAuthResponse` on success, or an `Error` on failure.
    pub async fn signup_with_face(
        &self,
        image_bytes: &[u8],
        custom_fields: Option<&HashMap<String, serde_json::Value>>,
    ) -> Result<FaceAuthResponse> {
        let image_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);
        let payload = SignupRequest {
            image_base64: &image_base64,
            custom_fields,
        };
        self.post("/auth/signup-face", &payload).await
    }

    /// Asynchronously authenticates an existing user by their face.
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - The raw byte array of the user's face image.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FaceAuthResponse` on success, or an `Error` on failure.
    pub async fn login_with_face(&self, image_bytes: &[u8]) -> Result<FaceAuthResponse> {
        let image_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);
        let payload = LoginRequest {
            image_base64: &image_base64,
        };
        self.post("/auth/login-face", &payload).await
    }

    /// Private helper function to handle POST requests.
    async fn post<T: Serialize>(&self, endpoint: &str, payload: &T) -> Result<FaceAuthResponse> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(payload).send().await?;

        if response.status().is_success() {
            // If the request was successful, deserialize the JSON into FaceAuthResponse.
            Ok(response.json::<FaceAuthResponse>().await?)
        } else {
            // If the API returned an error, deserialize it into our custom error type.
            let status = response.status();
            let error_response: HashMap<String, String> = response.json().await?;
            let message = error_response
                .get("detail")
                .cloned()
                .unwrap_or_else(|| "An unknown API error occurred.".to_string());

            Err(Error::Api { status, message })
        }
    }
}
