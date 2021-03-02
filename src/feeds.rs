use chrono::{DateTime, TimeZone, Utc};
use log::info;
use reqwest::{Result, Url};
use reqwest::header::USER_AGENT;
use rss::{ChannelBuilder, Item, ItemBuilder, Guid};
use scraper::{Html, Selector};

use std::collections::HashMap;

use crate::db;

pub fn refresh_feed(feed: &db::models::Feed) -> Result<()> {

    info!("get feed content '{}'", feed.title);

    let feed_items = db::items::get_items(feed.id.clone());

    let mut items = HashMap::new();

    for i in feed_items {
        items.insert(i.link.clone(), i);
    }

    let resp = reqwest::blocking::Client::new()
        .get(&*(feed.link.clone()))
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.16; rv:84.0) Gecko/20100101 Firefox/84.0",
        )
        .send()?;
    assert!(resp.status().is_success());

    let document = &resp.text()?;

    let document = Html::parse_document(document);

    let feed_title_selector = Selector::parse("head title").unwrap();

    let mut feed_title = String::from("");

    for fd in document.select(&feed_title_selector) {
        feed_title = fd.inner_html();
    }

    info!("found title '{}'", feed_title);

    let node_selector = Selector::parse(&feed.node_selector).unwrap();
    let title_selector: Option<Selector> = match &feed.title_selector {
        Some(ts) => Some(Selector::parse(ts).unwrap()),
        None => None
    };
    let link_selector: Option<Selector> = match &feed.link_selector {
        Some(ls) => Some(Selector::parse(ls).unwrap()),
        None => None
    };
    //let description_selector = None;

    for node in document.select(&node_selector) {
        let title_element = match title_selector {
            Some(ref ts) => node.select(ts).next(),
            None => Some(node),
        };

        if !title_element.is_some() {
            continue;
        }

        let title = match title_element.unwrap().text().next() {
            Some(t) => t.trim(),
            None => "",
        };

        if title == "" {
            continue;
        }

        let link_element = match link_selector {
            Some(ref ls) => node.select(ls).next(),
            None => Some(node),
        };

        let mut link_url = "";

        if link_element.is_some() {
            link_url = match link_element.unwrap().value().attr("href") {
                Some(l) => l,
                None => "",
            };
        }

        match items.get(link_url) {
            Some(_i) => {},
            None => {
                info!("title {}, link {}", title, link_url);

                // add an item
                let item = db::models::NewItem{
                    id: None,
                    feed: feed.id.clone(),
                    title: String::from(title),
                    link: String::from(link_url),
                    publication_date: chrono::Utc::now().naive_utc()
                };

                db::items::create_item(item);
            }
        };
    };

    Ok(())
}

pub fn to_rss(feed: &db::models::Feed, feed_items: Vec<db::models::Item>)  -> Result<String>{

    let mut items: Vec<Item> = Vec::new();

    for feed_item in feed_items {
        let item = ItemBuilder::default()
            .guid(Some(Guid{ value: feed_item.id.clone(), permalink: false }))
            .title(Some(feed_item.title.clone()))
            .link(Some(feed_item.link.clone()))
            .pub_date(Some(Utc.from_utc_datetime(&feed_item.publication_date).to_rfc2822()))
            .build()
            .unwrap();

        items.push(item)
    }

    let channel = ChannelBuilder::default()
        .title(feed.title.clone())
        .link(feed.link.clone())
        .last_build_date(Some(Utc::now().to_rfc2822()))
        .items(items)
        .build()
        .unwrap();

    Ok(channel.to_string())
}

// async fn hacker_news(feed: db::models::Feed) -> Result<str> {
//
//     let resp = reqwest::Client::new()
//         .get(&feed.link)
//         .header(
//             USER_AGENT,
//             "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.16; rv:84.0) Gecko/20100101 Firefox/84.0",
//         )
//         .send()
//         .await?;
//     assert!(resp.status().is_success());
//
//     let document = Html::parse_document(&resp.text().await?);
//
//     let feed_title_selector = Selector::parse("head title").unwrap();
//
//     let mut feed_title = String::from("");
//
//     for fd in document.select(&feed_title_selector) {
//         feed_title = fd.inner_html();
//     }
//
//     let node_selector = Selector::parse("html body div div div div article div p a").unwrap();
//     let title_selector: Option<Selector> = None; //Some(Selector::parse("a").unwrap());
//     let link_selector: Option<Selector> = None; //Some(Selector::parse("a").unwrap());
//     //let description_selector = None;
//
//     let mut items: Vec<Item> = Vec::new();
//
//     for node in document.select(&node_selector) {
//         let title_element = match title_selector {
//             Some(ref ts) => node.select(ts).next(),
//             None => Some(node),
//         };
//
//         if !title_element.is_some() {
//             continue;
//         }
//
//         let title = match title_element.unwrap().text().next() {
//             Some(t) => t,
//             None => "",
//         };
//
//         if title == "" {
//             continue;
//         }
//
//         let link_element = match link_selector {
//             Some(ref ls) => node.select(ls).next(),
//             None => Some(node),
//         };
//
//         let mut link_url = "";
//
//         if link_element.is_some() {
//             link_url = match link_element.unwrap().value().attr("href") {
//                 Some(l) => l,
//                 None => "",
//             };
//         }
//
//         println!("title {}, link {}", title, link_url);
//
//         let item = ItemBuilder::default()
//             .title(Some(String::from(title)))
//             .link(Some(String::from(link_url)))
//             .build()
//             .unwrap();
//
//         items.push(item)
//     }
//
//     let channel = ChannelBuilder::default()
//         .title(&feed_title)
//         .link("http://example.com")
//         .items(items)
//         .build()
//         .unwrap();
//
//     Ok(channel.to_string())
// }
