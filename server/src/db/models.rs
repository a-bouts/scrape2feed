use super::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub link: String,
    pub node_selector: String,
    pub title_selector: Option<String>,
    pub link_selector: Option<String>
}

#[derive(Queryable)]
pub struct Item {
    pub id: String,
    pub feed: String,
    pub title: String,
    pub link: String,
    pub publication_date: NaiveDateTime,
    pub description: Option<String>
}

#[derive(Insertable)]
#[table_name="feeds"]
pub struct NewFeed {
    pub id: Option<String>,
    pub title: String,
    pub link: String,
    pub node_selector: String,
    pub title_selector: Option<String>,
    pub link_selector: Option<String>
}

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem {
    pub id: Option<String>,
    pub feed: String,
    pub title: String,
    pub link: String,
    pub publication_date: NaiveDateTime,
    pub description: Option<String>
}
