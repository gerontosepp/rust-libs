//! Domain-neutral client for OpenAI-compatible chat completion endpoints.
//!
//! The crate knows nothing about records, playlists, users or persistence. It provides the
//! transport layer for applications that want to call a provider, use tool calls and validate
//! a connection. See the workspace README for the design and the crate spec.

#![forbid(unsafe_code)]

/// HTTP transport used by the client. The public chat API is added by issue #1.
pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    /// HTTP transport; the typed chat API is added by issue #1.
    pub fn transport(&self) -> &reqwest::Client {
        &self.http
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
