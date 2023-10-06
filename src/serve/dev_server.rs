use std::fs;
use warp::Filter;

// #[tokio::main]
pub async fn start_server(current_dir: &str) {
    // Define a warp filter that serves the index.html file

    let workspace_index = format!("{}/www/index.html", current_dir);

    let index = warp::path::end().map(move || {
        match fs::read_to_string(&workspace_index) {
            Ok(content) => warp::reply::html(content),
            Err(_) => warp::reply::html("Error reading index.html".to_string()),
        }
    });

    // Start the warp server
    warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}
