use scraper::{Html, Selector};
use crate::util::{Song, SearchParams};

#[derive(Clone, Debug, Hash, Eq)]
pub enum WDR {
    WDR1Live,
    WDR2,
    WDR3,
    WDR4,
    WDR5
}

impl std::cmp::PartialEq for WDR {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl WDR {
    pub fn get_url(&self) -> &str {
        match self {
            Self::WDR1Live => "https://www1.wdr.de/radio/1live/musik/playlist/index.jsp",
            Self::WDR2 => "https://www1.wdr.de/radio/wdr2/musik/playlist/index.jsp",
            Self::WDR3 => "https://www1.wdr.de/radio/wdr3/titelsuche-wdrdrei-104.jsp",
            Self::WDR4 => "https://www1.wdr.de/radio/wdr4/titelsuche-wdrvier-102.jsp",
            Self::WDR5 => "https://www1.wdr.de/radio/wdr5/musik/titelsuche-wdrfuenf-104.html",
        }
    }

    pub fn from_str(str: &str) -> Result<WDR, String> {
        let uppercase = str.to_uppercase();
        let without_spacing = uppercase.replace(" ", "");
        match &without_spacing[..] {
            "WDR1" => Ok(WDR::WDR1Live),
            "1LIVE" => Ok(WDR::WDR1Live),
            "WDR2" => Ok(WDR::WDR2),
            "WDR3" => Ok(WDR::WDR3),
            "WDR4" => Ok(WDR::WDR4),
            "WDR5" => Ok(WDR::WDR5),
            _ => Err(format!("Couldn't turn `{}` into a WDR Enum", &str))
        }
    }
}

pub async fn scrape_wdr(day: &str, wdr: &WDR) -> Vec<Song> {
    let mut playlist = vec![];

    for hour in 0..24 {
        let mut songs = scrape_wdr_playlist(&SearchParams {
            date: String::from(day),
            hour,
        }, wdr).await;
        
        println!("scraping for hour {}/{} => found {} songs", &hour + 1, 24, songs.len());

        playlist.append(&mut songs);
    }

    playlist   
}

async fn scrape_wdr_playlist(params: &SearchParams, wdr: &WDR) -> Vec<Song> {
    let url = wdr.get_url();
    let params = [
        ("playlistSearch_date", &String::from(&params.date)),
        ("playlistSearch_hours", &params.hour.to_string()),
        ("playlistSearch_minutes", &String::from("00")),
        ("submit", &String::from("suchen"))
    ];
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

    let row_selector = Selector::parse(".table tbody tr").expect("Couldn't select '.table tbody tr '");

    let mut playlist: Vec<_> = document.
        select(&row_selector)
        .map(|row| {
            let columns: Vec<_> = row.text().collect();

            let song = Song {
                title: row.select(&Selector::parse(".entry.title").expect("Couldn't find title of song")).next().unwrap().text().next().unwrap().to_string(),
                interprets: row.select(&Selector::parse(".entry.performer").expect("Couldn't find interprets of song")).next().unwrap().text().next().unwrap().to_string(),
                radio: wdr.clone()
            };

            song
    }).collect();

    playlist
}

