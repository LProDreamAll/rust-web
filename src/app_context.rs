use std::sync::Arc;

use crate::config::ServerConfig;

/// Application-wide shared dependencies/config live here.
#[derive(Clone)]
pub struct AppContext {
    pub server_config: ServerConfig,
}

#[derive(Debug)]
pub struct AppContextBuildError(&'static str);

impl std::fmt::Display for AppContextBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing required field: {}", self.0)
    }
}

impl std::error::Error for AppContextBuildError {}

pub struct AppContextBuilder {
    server_config: Option<ServerConfig>,
}

impl AppContext {
    pub fn builder() -> AppContextBuilder {
        AppContextBuilder::new()
    }

    /// Minimal entry point: build context from config.
    pub fn from_config(server_config: ServerConfig) -> Arc<Self> {
        Arc::new(Self { server_config })
    }
}

impl Default for AppContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AppContextBuilder {
    pub fn new() -> Self {
        Self {
            server_config: None,
        }
    }

    pub fn server_config(mut self, server_config: ServerConfig) -> Self {
        self.server_config = Some(server_config);
        self
    }

    pub fn build(self) -> Result<AppContext, AppContextBuildError> {
        let server_config = self
            .server_config
            .ok_or(AppContextBuildError("server_config"))?;

        Ok(AppContext { server_config })
    }
}
