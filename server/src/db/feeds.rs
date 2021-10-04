use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::ops::Deref;
use std::sync::{Arc};
use diesel::Connection;
use log::error;
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::db::models::*;
use crate::db::schema::feeds::dsl::*;


pub(crate) fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_feeds(cnx: Arc<Mutex<SqliteConnection>>) -> Vec<Feed> {

    let cnx = cnx.lock().await;

    feeds
        //.limit(20)
        .load::<Feed>(cnx.deref())
        .expect("Error loading feeds")
}

pub async fn get_feed(cnx: Arc<Mutex<SqliteConnection>>, feed_id: String) -> Option<Feed> {

    let cnx = cnx.lock().await;

    let mut fs = feeds.find(feed_id)
        .load::<Feed>(cnx.deref())
        .expect("Error finding feed");

    if fs.len() == 0 {
        return None
    }

    Some(fs.remove(0))
}

pub async fn delete_feed(cnx: Arc<Mutex<SqliteConnection>>, feed: Feed) -> usize {
    let cnx = cnx.lock().await;

    diesel::delete(&feed)
        .execute(cnx.deref())
        .expect("Error finding feed")
}

pub async fn create_feed(cnx: Arc<Mutex<SqliteConnection>>, feed: NewFeed) -> String {

    let cnx = cnx.lock().await;

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let new_feed = NewFeed {
        id: Some(uuid.clone()),
        title: feed.title,
        link: feed.link,
        node_selector: feed.node_selector,
        title_selector: feed.title_selector,
        link_selector: feed.link_selector,
    };

    diesel::insert_into(crate::db::schema::feeds::table)
        .values(&new_feed)
        .execute(cnx.deref())
        .expect("Error saving new feed");

    uuid
}

