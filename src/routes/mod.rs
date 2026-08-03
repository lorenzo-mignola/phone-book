mod contacts;
mod hello;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello::hello))
        .merge(contacts::contacts_router())
        .with_state(state)
}
