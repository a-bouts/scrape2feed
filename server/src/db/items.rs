use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::db::models::*;
use crate::db::schema::items::dsl::*;
use crate::db::feeds;

pub async fn get_items(cnx: Arc<Mutex<SqliteConnection>>, feed_id: String) -> Vec<Item> {
    let cnx = cnx.lock().await;

    items.filter(feed.eq(feed_id.clone()))
        .load::<Item>(cnx.deref())
        .expect("Error loading items")
}

pub async fn create_item(cnx: Arc<Mutex<SqliteConnection>>, item: NewItem) -> String {
    let cnx = cnx.lock().await;

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let new_item = NewItem {
        id: Some(uuid.clone()),
        feed: item.feed,
        title: item.title,
        link: item.link,
        description: item.description,
        publication_date: item.publication_date
    };

    diesel::insert_into(crate::db::schema::items::table)
        .values(&new_item)
        .execute(cnx.deref())
        .expect("Error saving new feed");

    uuid
}

pub async fn delete_items(cnx: Arc<Mutex<SqliteConnection>>, feed_id: String) -> usize {
    let cnx = cnx.lock().await;

    diesel::delete(items.filter(feed.eq(feed_id)))
        .execute(cnx.deref())
        .expect("Error deleting items")
}
