use diesel::prelude::*;



use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::db::models::*;
use crate::db::schema::items::dsl::*;


pub async fn get_items(cnx: Arc<Mutex<SqliteConnection>>, feed_id: String) -> Vec<Item> {
    let mut cnx = cnx.lock().await;

    items.filter(feed.eq(feed_id.clone()))
        .load::<Item>(cnx.deref_mut())
        .expect("Error loading items")
}

pub async fn create_item(cnx: Arc<Mutex<SqliteConnection>>, item: NewItem) -> String {
    let mut cnx = cnx.lock().await;

    let uuid = Uuid::new_v4().hyphenated().to_string();

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
        .execute(cnx.deref_mut())
        .expect("Error saving new feed");

    uuid
}

pub async fn delete_items(cnx: Arc<Mutex<SqliteConnection>>, feed_id: String) -> usize {
    let mut cnx = cnx.lock().await;

    diesel::delete(items.filter(feed.eq(feed_id)))
        .execute(cnx.deref_mut())
        .expect("Error deleting items")
}
