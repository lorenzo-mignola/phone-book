use axum::{Router, routing::get};

use crate::state::AppState;

use crate::features::contacts::index_handler;

pub(crate) fn ui_router() -> Router<AppState> {
    Router::new().route("/", get(index_handler))
}
