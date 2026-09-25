//! Subscription sign-ins already on this machine, so a user who signed in with
//! another tool can start without signing in again.

use std::path::Path;

use serde::Serialize;

use crate::codex;

/// A subscription sign-in found on disk that Aster can use as it is.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Login {
    pub provider: &'static str,
    pub name: &'static str,
    pub base_url: &'static str,
    pub account: Option<String>,
}

/// Every usable sign-in under `home`, most preferred first.
pub fn found(home: &Path) -> Vec<Login> {
    codex::load(home)
        .map(|auth| Login {
            provider: "codex",
            name: "ChatGPT",
            base_url: codex::CODEX_BASE_URL,
            account: codex::email(&auth),
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
#[path = "logins_tests.rs"]
mod tests;
