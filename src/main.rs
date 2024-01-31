//! This executable is used to scrape the links from reddit.sportshub.fan
//! and save them to database.  It also checks the stream links and saves them
//! to database.

const OPEN_TABS: usize = 10;

pub mod db;
pub mod models;
pub mod schema;

extern crate diesel;



use scraper::scrape_utils::start_scraping;


fn main() {
    start_scraping(OPEN_TABS);
}
