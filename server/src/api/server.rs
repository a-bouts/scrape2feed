use crate::api::v1;

use log::info;
use rocket_contrib::serve::StaticFiles;

pub fn init() -> rocket::Rocket {
    info!("Start server");
    rocket::ignite()
        .mount("/", StaticFiles::from("/static"))
        .mount("/api/v1", routes![v1::download, v1::get_feeds, v1::get_feed, v1::post_feed, v1::delete_feed, v1::get_feed_content])
}

