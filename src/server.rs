// cspell:ignore rerank
use std::{sync::Arc, time::Instant};

use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{app_context::AppContext, config::ServerConfig};

#[derive(Clone)]
pub struct AppState {
    pub context: Arc<AppContext>,
}

pub struct Server {
    config: ServerConfig,
    context: Arc<AppContext>,
}

impl Server {
    pub fn new(context: Arc<AppContext>) -> Self {
        let config = context.server_config.clone();
        Self { config, context }
    }

    /// Build the application router with all routes configured
    pub fn build_app(&self) -> Router {
        let state = Arc::new(AppState {
            context: self.context.clone(),
        });

        // Public routes - no authentication required
        let public_routes = Router::new().route("/health", get(health));

        // Protected routes - business logic endpoints
        let protected_routes = Router::new()
            .route("/rerank", post(rerank))
            .route("/sy/on/predict", post(predict_on_sy));

        // Combine all routes
        Router::new()
            .merge(public_routes)
            .merge(protected_routes)
            .with_state(state)
    }

    /// Convenience method for backward compatibility
    pub fn router(&self) -> Router {
        self.build_app()
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.config.addr();
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        println!("listening on http://{addr}");

        axum::serve(listener, self.build_app()).await?;
        Ok(())
    }
}

// ============================================================================
// Handler functions
// ============================================================================

async fn health(State(_state): State<Arc<AppState>>) -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RerankRequest {
    pub model: String,
    pub query: String,
    pub documents: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RerankResponse {
    pub results: Vec<RerankResult>,
}

#[derive(Debug, Serialize)]
pub struct RerankResult {
    pub index: usize,
    pub document: String,
    pub relevance_score: f32,
}

async fn rerank(State(_state): State<Arc<AppState>>, Json(body): Json<RerankRequest>) -> Response {
    // Mock implementation: return documents with dummy scores
    let results: Vec<RerankResult> = body
        .documents
        .into_iter()
        .enumerate()
        .map(|(index, document)| RerankResult {
            index,
            document,
            relevance_score: 1.0 / (index as f32 + 1.0), // Simple descending scores
        })
        .collect();

    (StatusCode::OK, Json(RerankResponse { results })).into_response()
}

/// Handler for /sy/on/predict - processes snappy-compressed JSON requests
/// Equivalent to Python's: orjson.loads(snappy.decompress(request.body).decode("utf-8"))
async fn predict_on_sy(State(_state): State<Arc<AppState>>, body: Bytes) -> Response {
    let start = Instant::now();
    let worker_id = std::process::id();
    info!(
        "请求成功 | 路径: /sy/on/predict | 方法: POST, work_id: {}",
        worker_id
    );
    // Step 1: Decompress snappy data
    let decompressed = match snap::raw::Decoder::new().decompress_vec(&body) {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Failed to decompress snappy data",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    // Step 2: Decode UTF-8
    let json_str = match std::str::from_utf8(&decompressed) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Failed to decode UTF-8",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    // Step 3: Parse JSON (using serde_json, which is very fast in Rust)
    let json_body: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Failed to parse JSON",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let elapsed = start.elapsed();
    info!("Processing completed in {:?}", elapsed);

    // Return the parsed data with processing info
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "worker_id": worker_id,
            "processing_time_ms": elapsed.as_millis(),
            "received_data": json_body
        })),
    )
        .into_response()
}
