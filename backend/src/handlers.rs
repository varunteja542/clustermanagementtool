use axum::{extract::Path, response::Html, Json};
use serde_json::json;

pub async fn get_clusters() -> Json<serde_json::Value> {
    Json(json!([
        { "id": 1, "name": "Cluster A" },
        { "id": 2, "name": "Cluster B" }
    ]))
}

pub async fn get_nodes(Path(_id): Path<u32>) -> Json<serde_json::Value> {
    Json(json!([
        { "id": "node-1", "status": "online" },
        { "id": "node-2", "status": "offline" }
    ]))
}

pub async fn register_node(Path(id): Path<u32>) -> Json<serde_json::Value> {
    Json(json!({ "message": format!("Node registered in cluster {}", id) }))
}

pub async fn delete_node(Path((cid, nid)): Path<(u32, String)>) -> Json<serde_json::Value> {
    Json(json!({ "message": format!("Node {} deleted from cluster {}", nid, cid) }))
}

pub async fn get_node_actions(Path(_id): Path<String>) -> Json<serde_json::Value> {
    Json(json!([
        { "action": "started", "time": "2025-04-10T12:00:00Z" }
    ]))
}

pub async fn serve_frontend() -> Html<String> {
    let html = std::fs::read_to_string("frontend/index.html")
        .unwrap_or_else(|_| "Could not load frontend".to_string());
    Html(html)
}
