use axum::{
    routing::{get, post, delete, get_service},
    Router,
};
use tower_http::services::ServeDir;
// Import ServiceExt so that map_request becomes available.
use tower::util::ServiceExt;
use hyper::{Body, Request, Server};

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Create a static file service for the "frontend" directory.
    // We convert the incoming Axum request (with axum::body::Body) into a hyper::Request.
    let static_service = get_service(ServeDir::new("frontend"))
        .map_request(|req: axum::http::Request<axum::body::Body>| {
            // Decompose the Axum request into its parts and body.
            let (parts, body) = req.into_parts();
            // Rebuild a hyper request using those parts.
            Request::from_parts(parts, body)
        })
        .handle_error(|err: std::io::Error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", err),
            )
        });

    // Build the application router.
    let app = Router::new()
        .route("/", get(handlers::serve_frontend))
        .route("/clusters", get(handlers::get_clusters))
        .route("/clusters/:id/nodes", get(handlers::get_nodes))
        .route("/clusters/:id/nodes", post(handlers::register_node))
        .route("/clusters/:cid/nodes/:nid", delete(handlers::delete_node))
        .route("/nodes/:id/actions", get(handlers::get_node_actions))
        // Mount the static service under /frontend.
        .nest_service("/frontend", static_service);

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);

    // Run the app using hyper's Server.
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
