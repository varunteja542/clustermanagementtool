use axum::{
    routing::{get, post, delete, get_service},
    Router,
};
use tower_http::services::ServeDir;
use hyper::Body;
use http::Request; // From the http crate

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Wrap ServeDir and convert request types.
    let static_service = get_service(ServeDir::new("frontend"))
        // Convert an Axum request (with axum::body::Body) into one with hyper::Body.
        .map_request(|req: axum::http::Request<axum::body::Body>| {
            // Break the request into its parts.
            let (parts, body) = req.into_parts();
            // Create a new request using the same parts and the same body.
            // Since axum::body::Body is an alias for hyper::Body in default configurations, this is a no‐op,
            // but this explicit conversion helps satisfy the trait bounds.
            Request::from_parts(parts, body)
        })
        .handle_error(|err: std::io::Error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", err),
            )
        });

    // Build our application router.
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

    // Use hyper::Server instead of axum::Server.
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
