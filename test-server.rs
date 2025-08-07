use std::net::SocketAddr;
use std::path::Path;
use warp::Filter;
use warp::http::Response;

#[tokio::main]
async fn main() {
    let port = 8000;
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();

    // Serve static files from current directory
    let static_files = warp::fs::dir(".")
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST"]));

    // Add proper headers for WASM
    let with_headers = warp::any()
        .map(|| {
            Response::builder()
                .header("Cross-Origin-Embedder-Policy", "require-corp")
                .header("Cross-Origin-Opener-Policy", "same-origin")
        });

    println!("Server running at http://localhost:{}/", port);
    println!("Open http://localhost:{}/demo.html to test the PlotScript engine", port);

    warp::serve(static_files)
        .run(addr)
        .await;
}