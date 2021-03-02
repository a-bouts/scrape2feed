use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use uuid::Uuid;
use crate::db::models::*;
use crate::db::schema::items::dsl::*;
use crate::db::feeds;

pub fn get_items(feed_id: String) -> Vec<Item> {
    let connection = feeds::establish_connection();
    items.filter(feed.eq(feed_id.clone()))
        .load::<Item>(&connection)
        .expect("Error loading items")
}

pub fn create_item(item: NewItem) -> String {
    let connection = feeds::establish_connection();

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let new_item = NewItem {
        id: Some(uuid.clone()),
        feed: item.feed,
        title: item.title,
        link: item.link,
        publication_date: item.publication_date
    };

    diesel::insert_into(crate::db::schema::items::table)
        .values(&new_item)
        .execute(&connection)
        .expect("Error saving new feed");

    uuid
}

pub fn delete_items(feed_id: String) -> usize {
    let connection = feeds::establish_connection();

    diesel::delete(items.filter(feed.eq(feed_id)))
        .execute(&connection)
        .expect("Error deleting items")
}
