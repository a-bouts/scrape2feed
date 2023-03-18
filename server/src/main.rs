#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate markup5ever_rcdom as rcdom;

#[macro_use]
extern crate diesel;


mod api;
mod feeds;
mod db;
mod downloader;


use std::sync::Arc;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::fs::FileServer;
use tokio::sync::Mutex;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[launch]
fn rocket() -> _ {
    std::env::var("RUST_LOG").map_err(|_| {
        std::env::set_var("RUST_LOG", "error,server=info");
    });
    env_logger::init();

    let mut cnx = db::feeds::establish_connection();
    cnx.run_pending_migrations(MIGRATIONS).unwrap();
    let cnx = Arc::new(Mutex::new(cnx));

    rocket::build()
        .manage(cnx)
        .mount("/", FileServer::from("/static"))
        .mount("/-", routes![api::healthz])
        .mount("/api/v1", routes![api::v1::download, api::v1::get_feeds, api::v1::get_feed, api::v1::post_feed, api::v1::delete_feed, api::v1::get_feed_content])
}

// fn main() {
//     let connection = db::feeds::establish_connection();
//
//     embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
//
//     if env::var_os("RUST_LOG").is_none() {
//         env::set_var("RUST_LOG", "scrape2feed=debug,rocket=debug");
//     }
//     // pretty_env_logger::init();
//
//     api::server::init().launch();
//     //hacker_news("https://gum-gum-streaming.com/one-piece-vostfr/").await;
// }
//
