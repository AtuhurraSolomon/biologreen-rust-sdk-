use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the successful JSON response from the BioLogreen API.
/// This is equivalent to the `FaceAuthResponse` class in the C# SDK.
#[derive(Debug, Deserialize)]
pub struct FaceAuthResponse {
    pub user_id: i64,
    pub is_new_user: bool,
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
}

/// Represents the JSON payload for a face signup request.
/// This will be serialized and sent as the body of the POST request.
#[derive(Debug, Serialize)]
pub(crate) struct SignupRequest<'a> {
    pub(crate) image_base64: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) custom_fields: Option<&'a HashMap<String, serde_json::Value>>,
}

/// Represents the JSON payload for a face login request.
#[derive(Debug, Serialize)]
pub(crate) struct LoginRequest<'a> {
    pub(crate) image_base64: &'a str,
}
