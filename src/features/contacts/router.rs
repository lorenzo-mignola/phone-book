use axum::{
    Router,
    routing::{get, post},
};

use super::routes;
use crate::state::AppState;

pub(crate) fn contacts_router() -> Router<AppState> {
    Router::new()
        .route("/contacts", get(routes::list_contacts))
        .route("/contacts/{id}", get(routes::get_contact))
        .route("/contacts", post(routes::save_contact))
}
