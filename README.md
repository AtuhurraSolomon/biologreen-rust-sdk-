Bio-Logreen Rust SDK
The official Rust SDK for the Bio-Logreen Facial Authentication API.

This SDK provides a simple, robust, and asynchronous client for interacting with the BioLogreen API from your Rust applications. It is designed to be efficient, type-safe, and easy to integrate.

Features
Modern & Asynchronous: Built with modern async/await for non-blocking, high-performance network operations.

Type-Safe: All API responses and requests are strongly typed, preventing common data-related bugs at compile time.

Robust Error Handling: Provides a comprehensive Error enum to easily distinguish between network failures and specific API errors.

Ergonomic API: A clean and simple client interface that mirrors the design of the other official BioLogreen SDKs.

Lightweight: Built on top of popular, well-maintained libraries like reqwest and serde.

Installation
This library is intended to be published to crates.io. To use it in your project, add the following line to your Cargo.toml file:

[dependencies]
biologreen = "0.1.0" # Or the latest version

You will also need to add tokio to run the async example:

[dependencies]
biologreen = "0.1.0"
tokio = { version = "1", features = ["full"] }

Quick Start: Usage Example
Here is a complete example for a simple console application that demonstrates how to sign up and log in a user.

// main.rs
use biologreen::{BioLogreenClient, Error};
use std::collections::HashMap;
use std::fs;

// The tokio::main attribute sets up the async runtime for our application.
#[tokio::main]
async fn main() {
    // It's recommended to load your API key from environment variables or a secure config.
    let api_key = "YOUR_SECRET_API_KEY".to_string();

    // The client is designed to be reused.
    // We pass `None` for the base_url to use the default for local testing: http://localhost:8000/v1
    let client = BioLogreenClient::new(api_key, None);

    // --- 1. Sign up a new user ---
    println!("Attempting to sign up a new user...");
    match fs::read("path/to/signup_image.jpg") {
        Ok(image_bytes) => {
            // Create some optional custom data to store with the user.
            let mut custom_fields = HashMap::new();
            custom_fields.insert("plan".to_string(), serde_json::json!("premium"));
            custom_fields.insert("internal_id".to_string(), serde_json::json!(12345));

            match client.signup_with_face(&image_bytes, Some(&custom_fields)).await {
                Ok(response) => {
                    println!("Signup Successful!");
                    println!("  User ID: {}", response.user_id);
                    println!("  Is New User: {}", response.is_new_user);
                    if let Some(fields) = response.custom_fields {
                        println!("  Custom Fields Returned: {:?}", fields);
                    }
                }
                Err(e) => handle_error(e),
            }
        }
        Err(e) => eprintln!("File Error reading signup image: {}", e),
    }

    println!("\n-----------------\n");

    // --- 2. Log in an existing user ---
    println!("Attempting to log in an existing user...");
    match fs::read("path/to/login_image.jpg") {
        Ok(image_bytes) => {
            match client.login_with_face(&image_bytes).await {
                Ok(response) => {
                    println!("Login Successful!");
                    println!("  User ID: {}", response.user_id);
                }
                Err(e) => handle_error(e),
            }
        }
        Err(e) => eprintln!("File Error reading login image: {}", e),
    }
}

/// A helper function to neatly handle our custom error types.
fn handle_error(e: Error) {
    match e {
        Error::Network(reqwest_err) => {
            // This is a network-level error (e.g., server down, DNS issue).
            eprintln!("Network Error: {}", reqwest_err);
        }
        Error::Api { status, message } => {
            // This is an error returned by the BioLogreen API
            // (e.g., face not found, invalid API key).
            eprintln!("API Error (Status {}): {}", status, message);
        }
    }
}

API Reference
BioLogreenClient
The main entry point for the SDK.

BioLogreenClient::new(api_key, base_url): Creates a new client. base_url is optional and defaults to the local testing environment.

Methods
async fn signup_with_face(&self, image_bytes: &[u8], custom_fields: Option<&HashMap<String, serde_json::Value>>)

Registers a new user. Takes a byte slice of an image and optional custom fields.

Returns a Result<FaceAuthResponse>.

async fn login_with_face(&self, image_bytes: &[u8])

Authenticates an existing user. Takes a byte slice of an image.

Returns a Result<FaceAuthResponse>.

Error Handling
The Result returned by the client methods contains the biologreen::Error enum, which has two variants:

Error::Network(reqwest::Error): For connection-level issues.

Error::Api { status, message }: For errors returned by the API backend.

Contributing
Suggestions and contributions are welcome. Please open an issue or a pull request on the GitHub repository to suggest changes.

License
This SDK is licensed under the MIT License with The Commons Clause. See the LICENSE file for more details.