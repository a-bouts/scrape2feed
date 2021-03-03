use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use uuid::Uuid;
use crate::db::models::*;
use crate::db::schema::feeds::dsl::*;

pub(crate) fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_feeds() -> Vec<Feed> {
    let connection = establish_connection();
    feeds
        .limit(20)
        .load::<Feed>(&connection)
        .expect("Error loading feeds")
}

pub fn get_feed(feed_id: String) -> Option<Feed> {
    let connection = establish_connection();
    let mut fs = feeds.find(feed_id)
        .load::<Feed>(&connection)
        .expect("Error finding feed");

    if fs.len() == 0 {
        return None
    }

    Some(fs.remove(0))
}

pub fn delete_feed(feed: Feed) -> usize {
    let connection = establish_connection();
    diesel::delete(&feed)
        .execute(&connection)
        .expect("Error finding feed")
}

pub fn create_feed(feed: NewFeed) -> String {
    let connection = establish_connection();

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
        .execute(&connection)
        .expect("Error saving new feed");

    uuid
}

