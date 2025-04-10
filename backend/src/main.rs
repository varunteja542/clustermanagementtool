use axum::{
    routing::{get, post, delete, get_service},
    Router,
};
use tower_http::services::ServeDir;
// Import the trait so we have access to `map_request`
use tower::util::ServiceExt;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Wrap ServeDir and convert the request type.
    let static_service = get_service(ServeDir::new("frontend"))
        .map_request(|req: axum::http::Request<axum::body::Body>| {
            // Break the request into parts.
            let (parts, body) = req.into_parts();
            // Create a new request using the same parts and body.
            axum::http::Request::from_parts(parts, body)
        })
        .handle_error(|err: std::io::Error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", err),
            )
        });

    // Build our application with API routes.
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

    // Use hyper::Server as Axum no longer exports a Server directly.
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
