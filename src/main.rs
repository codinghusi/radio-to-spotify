extern crate reqwest;
extern crate select;
extern crate tokio;

mod spotify;
mod util;
mod scraper;
mod config;

use util::Song;
use crate::config::load_config;
use std::{env, thread};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // get config
    let config = load_config();

    let yesterday = util::yesterday();
    
    env::set_var("CLIENT_ID", &config.auth.client_id);
    env::set_var("CLIENT_SECRET", &config.auth.client_secret);

    let spotify_credentials = aspotify::ClientCredentials::from_env().unwrap();
    let spotify_client = spotify::get_client(spotify_credentials, &config.auth.client_id).await;

    // let multi_progress = MultiProgress::new();
    // let progress_style = ProgressStyle::default_bar()
    //     .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    //     .progress_chars("#>-");
    //
    // let pb = multi_progress.add(ProgressBar::new(128));
    // pb.set_style(progress_style.clone());
    // let t1 = thread::spawn(move || {
    //     for i in 0..128 {
    //         pb.set_message(format!("item #{}", i + 1));
    //         pb.inc(1);
    //         thread::sleep(Duration::from_millis(100));
    //     }
    //     pb.finish_with_message("done");
    // });


    for playlist in config.playlists {
        println!("## Gathering Songs of {} ##", &playlist.radio);
        let wdr = scraper::WDR::from_str(&playlist.radio).expect("Couldn't find given radio name");
        let songs = scraper::scrape_wdr(&yesterday, &wdr).await;
        let songs = util::distinct_playlist(songs);

        println!(" == Finished == ");
        // publish on playlist
        println!(" ## Publishing them on Spotify (playlist: {}) ## ", &playlist.spotify_playlist_id);
        spotify::publish(&spotify_client, &playlist.spotify_playlist_id, &songs).await;
    }

    // t1.join();
    // t2.join();
    // multi_progress.clear().unwrap();
    
}

