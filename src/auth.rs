use crate::error::Error;
use std::process::Command;

/// Load the Linear API key from environment or macOS Keychain.
pub fn load_api_key() -> Result<String, Error> {
    // 1. Check environment variable
    if let Ok(key) = std::env::var("LINEAR_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    // 2. Try macOS Keychain
    if cfg!(target_os = "macos") {
        if let Ok(key) = keychain_get("linear-api-key") {
            return Ok(key);
        }
    }

    Err(Error::Auth)
}

fn keychain_get(service: &str) -> Result<String, Error> {
    let output = Command::new("security")
        .args(["find-generic-password", "-s", service, "-w"])
        .output()
        .map_err(|_| Error::Auth)?;

    if output.status.success() {
        let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if key.is_empty() {
            return Err(Error::Auth);
        }
        Ok(key)
    } else {
        Err(Error::Auth)
    }
}
