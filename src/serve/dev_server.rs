use axum::body::Full;
use axum::{response::IntoResponse, routing::get, Router};
use std::{fs, net::SocketAddr, path::Path};
use tracing::{error, info};

async fn static_asset(asset: impl AsRef<Path>) -> impl IntoResponse {
    let asset = asset.as_ref();
    let Ok(contents) = std::fs::read(asset) else {
        panic!("file does not exist: {}", asset.display());
    };

    let content_type = match asset.extension() {
        Some(ext) if ext == "js" => "text/javascript",
        Some(ext) if ext == "wasm" => "application/wasm",
        _ => "text/html", // Default to HTML for other extensions or no extension
    };

    axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header("content-type", content_type)
        .body(Full::from(contents))
        .unwrap()
}

pub async fn start_server(current_dir: impl AsRef<Path>) {
    let www_dir = current_dir.as_ref().join("www");
    let mut routes = Router::new();

    if let Ok(entries) = fs::read_dir(&www_dir) {
        //add routes for all static files
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let route = format!(
                    "/{}",
                    path.strip_prefix(&www_dir).unwrap().display()
                );
                // set root to index.html
                if path.file_name().unwrap() == "index.html" {
                    routes = routes
                        .route("/", get(move || static_asset(path.clone())));
                } else {
                    routes = routes
                        .route(&route, get(move || static_asset(path.clone())));
                }
                info!("Found Entry: {}", entry.path().display());
            }
        }
    } else {
        error!("Failed to read www directory");
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    info!("Server is running at http://127.0.0.1:3030");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap_or_else(|e| error!("Server error: {:?}", e));
}
