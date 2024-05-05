use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MartialArtsModel {
    pub id: Uuid,
    pub title: String,
    pub category: String,
    pub description: String,
    pub origin: Option<String>,
    pub founded_date: Option<String>
}