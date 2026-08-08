use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    features::{contacts, ui::ui_router},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    let api_router = Router::new().merge(contacts::router());

    Router::new()
        .nest("/api", api_router)
        .merge(ui_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}
