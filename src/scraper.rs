use std::collections::HashSet;
use scraper::{Html, Selector};
use chrono::{Utc};
use crate::util::{Song, SearchParams};

pub async fn scrape() -> Vec<Song> {
    let playlist = radio_to_playlist(&yesterday()).await;
    println!("got {} songs", playlist.len());
    println!("uploading them to spotify...");
    playlist
}

async fn radio_to_playlist(day: &str) -> Vec<Song> {
    let mut playlist = vec![];

    for hour in 0..24 {
        let mut songs = scrape_playlist(&SearchParams {
            date: String::from(day),
            hour,
        }).await;
        
        println!("scraping for hour {}/{} => found {} songs", &hour + 1, 24, songs.len());

        playlist.append(&mut songs);
    }

    playlist   
}

fn yesterday() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

async fn scrape_playlist(params: &SearchParams) -> Vec<Song> {
    let url = "https://www1.wdr.de/radio/wdr2/musik/playlist/index.jsp";
    let params = [
        ("playlistSearch_date", &String::from(&params.date)),
        ("playlistSearch_hours", &params.hour.to_string()),
        ("playlistSearch_minutes", &String::from("00")),
        ("submit", &String::from("suchen"))
    ];
    println!("params: {:?}", &params);
    let client = reqwest::Client::new();
    let response = client.post(url)
                         .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:88.0) Gecko/20100101 Firefox/88.0")
                         .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
                         .form(&params)
                         .send()
                         .await
                         .expect("Couldn't fetch playlist");
    assert!(response.status().is_success());

    let body = response.text().await.expect("Couldn't get the response text");
    let document = Html::parse_document(&body);
    
    let row_selector = Selector::parse("tbody tr").expect("Couldn't select 'tbody tr'");

    let mut playlist = Vec::new();

    for row in document.select(&row_selector) {
        let columns = row.text().collect::<Vec<_>>();

        let song = Song {
            time_str: format!("{} {}", columns[0].trim(), columns[1].trim()),
            title: String::from(columns[4].trim()),
            interprets: String::from(columns[6].trim())
        };

        playlist.push(song);
    }

    playlist
}

fn request_body(params: &[(&str, &String)]) -> String {
    let mut result = String::new();
    let mut separator = "";
    for param in params {
        let (key, value) = param;
        result = format!("{}{}{}={}", result, &separator, &key, &value);
        separator = "&";
    }
    result
}

pub fn distinct_playlist(playlist: Vec<Song>) -> Vec<Song> {
    let mut new = HashSet::new();
    for song in playlist {
        new.insert(song);
    }
    new.into_iter().collect()
}