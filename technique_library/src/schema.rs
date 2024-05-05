use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMartialArtsSchema {
    pub title: String,
    pub category: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub founded_date: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMartialArtsSchema {
    pub title: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub origin: Option<String>,
    pub founded_date: Option<String>
}