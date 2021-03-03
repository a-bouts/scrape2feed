use rocket_contrib::json::Json;

use crate::api::model::*;
use crate::{db, feeds, downloader};
use chrono::format::format;
use rocket::http::Status;
use std::io::Cursor;
use rocket::response::Result;
use reqwest::header::USER_AGENT;

#[get("/download?<url>")]
pub fn download(url: String) -> Result<'static> {

    match downloader::download(url) {
        Ok(t) => Ok(rocket::response::Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(t))
            .finalize()),
        Err(_e) => Err(Status::InternalServerError)
    }
}

#[get("/feeds")]
pub fn get_feeds() -> Json<Vec<Feed>> {
    let feeds = db::feeds::get_feeds();

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
pub fn get_feed(id: String) -> Json<Option<Feed>> {
    let feed = db::feeds::get_feed(id.clone());

    let items = db::items::get_items(id.clone());

    let mut items_model = Vec::new();

    for item in items {
        items_model.push(Item{
            id: item.id,
            title: item.title,
            link: item.link,
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
pub fn get_feed_content(id: String) -> Result<'static> {
    let feed = db::feeds::get_feed(id.clone());

    match feed {
        Some(f) => {
            feeds::refresh_feed(&f);

            match feeds::to_rss(&f, db::items::get_items(id)) {
                Ok(r) => Ok(rocket::response::Response::build()
                    .status(Status::Ok)
                    .sized_body(Cursor::new(r))
                    .finalize()),
                Err(_e) => Err(Status::InternalServerError)
            }
        },
        None => Err(Status::NotFound)
    }
}

#[post("/feeds", format = "application/json", data = "<feed>")]
pub fn post_feed(feed: Json<Feed>) -> String {
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

    return db::feeds::create_feed(new_feed);
}

#[delete("/feeds/<id>")]
pub fn delete_feed(id: String) -> Result<'static> {

    let feed = db::feeds::get_feed(id.clone());

    match feed {
        Some(f) => {
            db::items::delete_items(id.clone());
            db::feeds::delete_feed(f);
            Ok(rocket::response::Response::build()
               .status(Status::NoContent)
               .finalize())
        },
        None => Err(Status::NotFound),
    }
}
