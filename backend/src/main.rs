use axum::{
    routing::{get, post, delete},
    Router,
};
use tower_http::services::ServeDir;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Build our Axum app with routes.
    let app = Router::new()
        // Serve the index.html at the root path
        .route("/", get(handlers::serve_frontend))
        // API endpoints for cluster & node management
        .route("/clusters", get(handlers::get_clusters))
        .route("/clusters/:id/nodes", get(handlers::get_nodes))
        .route("/clusters/:id/nodes", post(handlers::register_node))
        .route("/clusters/:cid/nodes/:nid", delete(handlers::delete_node))
        .route("/nodes/:id/actions", get(handlers::get_node_actions))
        // Serve static files (CSS, JS) at /frontend
        .nest_service("/frontend", ServeDir::new("frontend"));

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
