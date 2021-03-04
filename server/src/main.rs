#![feature(proc_macro_hygiene, decl_macro)]

extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
extern crate rss;
extern crate tokio;

#[macro_use]
extern crate rocket;
extern crate html5ever;
extern crate rocket_contrib;
extern crate rocket_http;
extern crate markup5ever_rcdom as rcdom;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;

extern crate uuid;

mod api;
mod feeds;
mod db;
mod downloader;

use std::env;

embed_migrations!("./migrations");

fn main() {
    let connection = db::feeds::establish_connection();

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "scrape2feed=debug,rocket=debug");
    }
    // pretty_env_logger::init();

    api::server::init().launch();
    //hacker_news("https://gum-gum-streaming.com/one-piece-vostfr/").await;
}

