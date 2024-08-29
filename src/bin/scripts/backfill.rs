use discit_backend::{errors::AppError, logger, models::disc::Disc};
use mongoose::Model;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
struct DiscJson {
    pub name: String,
    pub brand: String,
    pub category: String,
    pub speed: String,
    pub glide: String,
    pub turn: String,
    pub fade: String,
    pub stability: String,
    pub link: String,
    pub pic: String,
    pub name_slug: String,
    pub brand_slug: String,
    pub category_slug: String,
    pub stability_slug: String,
    pub color: String,
    pub background_color: String,
}

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    logger::init()?;
    let string = read_to_string("./.backup/discs.json").map_err(AppError::internal_server_error)?;
    let data =
        serde_json::from_str::<Vec<DiscJson>>(&string).map_err(AppError::internal_server_error)?;
    let mut discs = vec![];
    for disc_json in data {
        let disc = Disc {
            name: disc_json.name,
            brand: disc_json.brand,
            category: disc_json.category,
            speed: disc_json.speed.parse().map_err(AppError::bad_request)?,
            glide: disc_json.glide.parse().map_err(AppError::bad_request)?,
            turn: disc_json.turn.parse().map_err(AppError::bad_request)?,
            fade: disc_json.fade.parse().map_err(AppError::bad_request)?,
            stability: disc_json.stability,
            link: disc_json.link,
            pic: disc_json.pic,
            name_slug: disc_json.name_slug,
            brand_slug: disc_json.brand_slug,
            category_slug: disc_json.category_slug,
            stability_slug: disc_json.stability_slug,
            color: disc_json.color,
            background_color: disc_json.background_color,
            ..Default::default()
        };
        discs.push(disc);
    }
    let inserted = Disc::bulk_insert(&discs)
        .await
        .map_err(AppError::bad_request)?;
    tracing::info!("docs inserted: {:#?}", inserted.inserted_ids.len());
    Ok(())
}
