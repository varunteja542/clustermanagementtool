use axum::{
    routing::{get, post, delete, get_service},
    Router,
};
use tower_http::services::ServeDir;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Wrap ServeDir with get_service and a simple error handler.
    let static_service = get_service(ServeDir::new("frontend")).handle_error(|err: std::io::Error| async move {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    });

    // Build the application with API routes and serve static files at /frontend.
    let app = Router::new()
        .route("/", get(handlers::serve_frontend))
        .route("/clusters", get(handlers::get_clusters))
        .route("/clusters/:id/nodes", get(handlers::get_nodes))
        .route("/clusters/:id/nodes", post(handlers::register_node))
        .route("/clusters/:cid/nodes/:nid", delete(handlers::delete_node))
        .route("/nodes/:id/actions", get(handlers::get_node_actions))
        .nest_service("/frontend", static_service);

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
