pub mod handlers;

use axum::{Router, routing::get};
use handlers::hello;

pub fn router() -> Router {
    Router::new().route("/", get(hello))
}
