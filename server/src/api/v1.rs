
use crate::api::model::*;
use crate::{db, feeds, downloader};
use log::{info, error};
use chrono::format::format;
use rocket::http::Status;
use std::io::Cursor;
use std::sync::Arc;
use diesel::SqliteConnection;
use rocket::response::Result;
use reqwest::header::USER_AGENT;
use rocket::serde::{Serialize, json::Json};
use rocket::State;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[get("/download?<url>")]
pub fn download(url: String) -> (Status, String) {

    match downloader::download(url) {
        Ok(t) => (Status::Ok, t),
        Err(_e) => (Status::InternalServerError, "".to_string())
    }
}

#[get("/feeds")]
pub async fn get_feeds(cnx: &State<Arc<Mutex<SqliteConnection>>>) -> Json<Vec<Feed>> {
    let feeds = db::feeds::get_feeds(cnx.inner().clone()).await;

    let mut model: Vec<Feed> = Vec::new();

    for feed in feeds {
        model.push(Feed {
            id: Some(feed.id),
            title: feed.title,
            link: feed.link,
            node_selector: feed.node_selector,
            title_selector: feed.title_selector,
            link_selector: feed.link_selector,
            items: Vec::new()
        })
    }

    Json(model)
}

#[get("/feeds/<id>/debug")]
pub async fn get_feed(id: String, cnx: &State<Arc<Mutex<SqliteConnection>>>) -> Json<Option<Feed>> {

    let feed = db::feeds::get_feed(cnx.inner().clone(), id.clone()).await;

    let items = db::items::get_items(cnx.inner().clone(), id.clone()).await;

    let mut items_model = Vec::new();

    for item in items {
        items_model.push(Item{
            id: item.id,
            title: item.title,
            link: item.link,
            description: item.description,
            publication_date: item.publication_date
        })
    }

    match feed {
        Some(f) => Json(Some(Feed {
            id: Some(f.id),
            title: f.title,
            link: f.link,
            node_selector: f.node_selector,
            title_selector: f.title_selector,
            link_selector: f.link_selector,
            items: items_model
        })),
        None => Json(None)
    }
}

#[get("/feeds/<id>")]
pub async fn get_feed_content(id: String, cnx: &State<Arc<Mutex<SqliteConnection>>>) -> (Status, String) {

    let start = Instant::now();

    let feed = db::feeds::get_feed(cnx.inner().clone(), id.clone()).await;


    match feed {
        Some(f) => {

            feeds::refresh_feed(cnx.inner().clone(), &f).await;

            match feeds::to_rss(&f, db::items::get_items(cnx.inner().clone(), id).await) {
                Ok(r) => {
                    info!("Get feed content '{}' in {}", &f.title, Instant::now().duration_since(start).as_secs());
                    (Status::Ok, r)
                },
                Err(e) => {
                    error!("Error getting feed content '{}' : {}", &f.title, e);
                    (Status::InternalServerError, "".to_string())
                }
            }
        },
        None => (Status::NotFound, "".to_string())
    }
}

#[post("/feeds", format = "application/json", data = "<feed>")]
pub async fn post_feed(feed: Json<Feed>, cnx: &State<Arc<Mutex<SqliteConnection>>>) -> String {
    let new_feed = db::models::NewFeed {
        id: None,
        title: feed.title.clone(),
        link: feed.link.clone(),
        node_selector: feed.node_selector.clone(),
        title_selector: match feed.title_selector.clone() {
            Some(t) => {
                if t.len() == 0 {
                    None
                } else {
                    Some(t)
                }
            },
            None => None
        },
        link_selector: match feed.link_selector.clone() {
            Some(t) => {
                if t.len() == 0 {
                    None
                } else {
                    Some(t)
                }
            },
            None => None
        },
    };

    return db::feeds::create_feed(cnx.inner().clone(), new_feed).await;
}

#[delete("/feeds/<id>")]
pub async fn delete_feed(id: String, cnx: &State<Arc<Mutex<SqliteConnection>>>) -> Status {

    let feed = db::feeds::get_feed(cnx.inner().clone(), id.clone()).await;

    match feed {
        Some(f) => {
            db::items::delete_items(cnx.inner().clone(), id.clone()).await;
            db::feeds::delete_feed(cnx.inner().clone(), f).await;
            Status::NoContent
        },
        None => Status::NotFound,
    }
}
