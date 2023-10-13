use axum::body::Full;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Router};
use std::{net::SocketAddr, path::Path, sync::Arc};
use tracing::{error, info};

async fn static_asset(asset: impl AsRef<Path>) -> impl IntoResponse {
    let asset = asset.as_ref();
    let Ok(contents) = std::fs::read(asset) else {
        panic!("file does not exist: {}", asset.display());
    };
    // todo: This is super lazy, use asset.extension()
    let path_str = asset.to_string_lossy().to_string();
    let content_type = if path_str.ends_with(".js") {
        "text/javascript"
    } else if path_str.ends_with(".wasm") {
        "application/wasm"
    } else {
        "text/html"
    };

    axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header("content-type", content_type)
        .body(Full::from(contents))
        .unwrap()
}

pub async fn start_server(current_dir: impl AsRef<Path>) {
    let routes = Router::new()
        .route("/", get(|| static_asset("./www/index.html")))
        .route("/client.js", get(|| static_asset("./www/client.js")))
        .route(
            "/client_bg.wasm",
            get(|| static_asset("./www/client_bg.wasm")),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap_or_else(|e| error!("Server error: {:?}", e));

    // Log the HTTP link
    info!("Server is running at http://127.0.0.1:3030");
}
