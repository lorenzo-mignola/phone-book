pub mod handlers;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::hello))
        .with_state(state)
}
