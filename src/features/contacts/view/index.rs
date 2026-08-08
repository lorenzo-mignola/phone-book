use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::{error::AppError, features::contacts::dto::ContactDto, state::AppState};

use crate::features::contacts::service;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate {
    contacts: Vec<ContactDto>,
}

pub(crate) async fn index_handler(
    State(state): State<AppState>,
) -> Result<IndexTemplate, AppError> {
    let contacts = service::contacts::find_all(&state).await?;

    Ok(IndexTemplate { contacts })
}
