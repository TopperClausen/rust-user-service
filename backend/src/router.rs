use axum::{
  routing::get,
  Router,
  Json
};
use std::convert::Infallible;

pub fn setup_router() -> Router {
  return Router::new()
    .route("/", get(handler));
}

#[derive(serde::Serialize)]
struct Message {
  message: String
}

async fn handler() -> Result<Json<Message>, Infallible> {
  Ok(Json(Message { message: format!("Hello from Rust") }))
}