use axum::{
    routing::{get, post, delete, get_service},
    Router,
};
use tower_http::services::ServeDir;
// Import ServiceExt from Tower (enabled via features)
use tower::util::ServiceExt;
// Import Hyper types
use hyper::{Body, Request, Server};

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Wrap ServeDir for serving the "frontend" directory and map the request type.
    let static_service = get_service(ServeDir::new("frontend"))
        .map_request(|req: axum::http::Request<axum::body::Body>| {
            let (parts, body) = req.into_parts();
            // Convert to a Hyper request (axum::body::Body is by default hyper::Body)
            Request::from_parts(parts, body)
        })
        .handle_error(|err: std::io::Error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", err),
            )
        });

    // Build the router with API endpoints and the static file service.
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

    // Use hyper::Server to run the app.
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
