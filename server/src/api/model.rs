use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub id: Option<String>,
    pub title: String,
    pub link: String,
    pub node_selector: String,
    pub title_selector: Option<String>,
    pub link_selector: Option<String>,
    #[serde(default = "default_items")]
    pub items: Vec<Item>,
}

fn default_items() -> Vec<Item> {
    Vec::new()
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub publication_date: NaiveDateTime
}

