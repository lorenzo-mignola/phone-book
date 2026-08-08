use axum::{
    Router,
    routing::{get, post, put},
};

use super::routes;
use crate::state::AppState;

pub(crate) fn contacts_router() -> Router<AppState> {
    Router::new()
        .route("/contacts", get(routes::contacts::list_contacts))
        .route("/contacts/{id}", get(routes::contacts::get_contact))
        .route("/contacts/{id}", put(routes::contacts::update_contact))
        .route("/contacts", post(routes::contacts::save_contact))
}
