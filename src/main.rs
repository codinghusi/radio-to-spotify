extern crate reqwest;
extern crate select;
extern crate tokio;

mod spotify;
mod util;
mod scraper;

use util::Song;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let playlist = scraper::scrape().await;
        let playlist = scraper::distinct_playlist(playlist);
        println!("playlist: {:?}", playlist);
        spotify::publish(&playlist).await;
    });
}

