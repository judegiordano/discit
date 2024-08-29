use axum::{extract::Path, response::IntoResponse, Json};
use meme_cache::{get, set};
use mongoose::{doc, types::ListOptions, Model};

use crate::{
    errors::AppError,
    models::disc::{Disc, DiscDto},
    types::{ApiResponse, ONE_HOUR_MS},
};

const LIST_ALL_DISCS_KEY: &str = "all_discs";

pub async fn list_all_discs() -> ApiResponse {
    if let Some(cache) = get::<Vec<DiscDto>>(LIST_ALL_DISCS_KEY).await {
        return Ok(Json(cache).into_response());
    }
    let discs = Disc::list(
        doc! {},
        ListOptions {
            limit: 0,
            sort: doc! { "name": 1 },
            ..Default::default()
        },
    )
    .await
    .map_err(AppError::not_found)?;
    let disc_dtos = discs.iter().map(Disc::dto).collect::<Vec<_>>();
    set(LIST_ALL_DISCS_KEY, &disc_dtos, ONE_HOUR_MS).await;
    Ok(Json(disc_dtos).into_response())
}

pub async fn read_by_id(id: Path<String>) -> ApiResponse {
    let disc_id = id.to_string();
    if let Some(cache) = get::<DiscDto>(&disc_id).await {
        return Ok(Json(cache).into_response());
    }
    let disc = Disc::read_by_id(&disc_id)
        .await
        .map_err(AppError::not_found)?;
    let disc_dto = disc.dto();
    set(&disc_id, &disc_dto, ONE_HOUR_MS).await;
    Ok(Json(disc_dto).into_response())
}
