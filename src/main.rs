// src/main.rs
use anyhow;
use axum::{
    Json,
    body::Body,
    extract::Path,
    http::{HeaderMap, HeaderValue, Response, StatusCode, header},
    response::IntoResponse,
    routing::{Router, get},
};
use mime_guess::from_path;
use percent_encoding::percent_decode_str;
use rust_embed::RustEmbed;
use std::{borrow::Cow, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[derive(RustEmbed)]
#[folder = "frontend/dist"] // <— built assets live here
#[prefix = "/"] // assets will appear with a leading '/'
struct Assets;

#[tokio::main]
async fn main() {
    // API routes
    let api = Router::new()
        .route("/hello", get(api_hello))
        .route("/health", get(|| async { "ok" }));

    // App routes (static + SPA fallback)
    let app = Router::new().nest("/api", api).fallback(static_handler);

    let app = app.layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new()) // gzip/br for API responses
            .layer(TraceLayer::new_for_http()),
    );

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Listneing failed");

    axum::serve(listener, app).await.expect("Server failed");
}

#[derive(serde::Serialize)]
struct Hello {
    message: String,
}

async fn api_hello() -> Json<Hello> {
    Json(Hello {
        message: "Hi from Axum + rust-embed!".into(),
    })
}

/// Serve embedded static assets with SPA fallback:
/// - If path is an asset -> serve it
/// - If path contains no dot or asset not found -> serve /index.html
async fn static_handler(Path(mut path): Path<String>) -> impl IntoResponse {
    // Decode URL-encoded path segments (e.g., %20)
    path = percent_decode_str(&path).decode_utf8_lossy().to_string();

    // Normalize: path "" or "/" -> "index.html"
    let path_is_file_like = path.contains('.');
    let candidate = if path.is_empty() || path == "/" {
        "index.html".to_string()
    } else if path_is_file_like {
        path.trim_start_matches('/').to_string()
    } else {
        // SPA route (e.g. /dashboard) -> index.html
        "index.html".to_string()
    };

    match get_asset_or_index(&candidate).await {
        Ok(resp) => resp,
        Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}

async fn get_asset_or_index(candidate: &str) -> Result<Response<Body>, anyhow::Error> {
    // Try asset first
    if let Some(file) = Assets::get(candidate) {
        return Ok(build_file_response(candidate, file.data));
    }

    // Fallback to SPA shell
    if let Some(index) = Assets::get("index.html") {
        return Ok(build_file_response("index.html", index.data));
    }

    Err(anyhow::anyhow!("No asset and no index.html found"))
}

fn build_file_response(path: &str, file: Cow<'static, [u8]>) -> Response<Body> {
    let mime = from_path(path).first_or_octet_stream();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );

    // Simple caching: long cache for hashed assets, short for index.html
    if path == "index.html" {
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    } else {
        headers.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    }

    // If you want ETag support, you could hash `file` (e.g., blake3) and compare
    // request If-None-Match header here and return 304.

    let mut builder = Response::builder().status(StatusCode::OK);

    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    builder.body(Body::from(file.into_owned())).unwrap()
}
