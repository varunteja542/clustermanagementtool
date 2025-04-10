mod handlers;
mod models;

use axum::{
    routing::{get, post, delete},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/clusters", get(handlers::get_clusters))
        .route("/clusters/:id/nodes", get(handlers::get_nodes).post(handlers::register_node))
        .route("/clusters/:id/nodes/:node_id", delete(handlers::delete_node))
        .route("/nodes/:id/actions", get(handlers::get_node_actions))
        .route("/", get(handlers::serve_frontend));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
