use chrono::Utc;
use mongoose::{doc, types::MongooseError, DateTime, IndexModel, IndexOptions, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disc {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub speed: f32,
    pub glide: f32,
    pub turn: f32,
    pub fade: f32,
    pub stability: String,
    pub link: String,
    pub pic: String,
    pub name_slug: String,
    pub brand_slug: String,
    pub category_slug: String,
    pub stability_slug: String,
    pub color: String,
    pub background_color: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscDto {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub speed: f32,
    pub glide: f32,
    pub turn: f32,
    pub fade: f32,
    pub stability: String,
    pub link: String,
    pub pic: String,
    pub name_slug: String,
    pub brand_slug: String,
    pub category_slug: String,
    pub stability_slug: String,
    pub color: String,
    pub background_color: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Disc {
    #[allow(dead_code)]
    pub async fn migrate() -> Result<Vec<String>, MongooseError> {
        let indexes = vec![IndexModel::builder()
            .keys(doc! {})
            .options(IndexOptions::builder().build())
            .build()];
        let result = Self::create_indexes(&indexes).await?;
        Ok(result.index_names)
    }

    pub fn dto(&self) -> DiscDto {
        DiscDto {
            id: self.id.clone(),
            name: self.name.clone(),
            brand: self.brand.clone(),
            category: self.category.clone(),
            speed: self.speed,
            glide: self.glide,
            turn: self.turn,
            fade: self.fade,
            stability: self.stability.clone(),
            link: self.link.clone(),
            pic: self.pic.clone(),
            name_slug: self.name_slug.clone(),
            brand_slug: self.brand_slug.clone(),
            category_slug: self.category_slug.clone(),
            stability_slug: self.stability_slug.clone(),
            color: self.color.clone(),
            background_color: self.background_color.clone(),
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}

impl Default for Disc {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            name: String::default(),
            brand: String::default(),
            category: String::default(),
            speed: Default::default(),
            glide: Default::default(),
            turn: Default::default(),
            fade: Default::default(),
            stability: String::default(),
            link: String::default(),
            pic: String::default(),
            name_slug: String::default(),
            brand_slug: String::default(),
            category_slug: String::default(),
            stability_slug: String::default(),
            color: String::default(),
            background_color: String::default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Disc {}
