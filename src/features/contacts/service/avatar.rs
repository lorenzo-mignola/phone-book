use crate::{
    error::AppError,
    features::contacts::{
        dto::Svg,
        repository::{ContactWithNumbers, contacts},
    },
    state::AppState,
};

pub(crate) async fn get_avatar(id: i32, state: &AppState) -> Result<Svg, AppError> {
    let ContactWithNumbers { contact, .. } = contacts::find_by_id(&state.db, id).await?;

    let user_name = format!(
        "{}_{}",
        contact.first_name,
        contact.last_name.unwrap_or_default()
    );

    fetch_avatar(user_name).await
}

async fn fetch_avatar(user_name: String) -> Result<Svg, AppError> {
    let url = format!("https://api.dicebear.com/10.x/blobs/svg?seed={}", user_name);

    let response = reqwest::get(url).await.map_err(|_| AppError::NotFound)?;

    let svg = response
        .text()
        .await
        .map_err(|_| AppError::NotFound)?
        .into();

    Ok(svg)
}
