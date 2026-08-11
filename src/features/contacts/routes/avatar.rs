use axum::{
    body::Body,
    extract::{Path, State},
    response::Html,
};

use crate::{error::AppError, features::contacts::service::avatar, state::AppState};

pub(crate) async fn get_contact_avatar(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Html<Body>, AppError> {
    let svg = avatar::get_avatar(id, &state).await?;

    Ok(Html(svg.into()))
}
