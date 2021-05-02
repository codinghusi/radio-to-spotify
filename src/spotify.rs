use crate::Song;
use aspotify::{Client, ClientCredentials, Scope};
use dotenv::dotenv;
use std::env;
use std::io;
use std::fs;
use std::cmp::min;

fn load_refresh_token() -> Option<String> {
    fs::read_to_string(".refresh_token").ok()
}

fn save_refresh_token(refresh_token: &String) {
    fs::write(".refresh_token", refresh_token).unwrap();
}

async fn prompt_refresh_token(client: &Client, client_id: &String) {
    let scopes = vec![
        Scope::PlaylistModifyPublic,
        Scope::UserFollowModify
    ];
    let (url, state) = aspotify::authorization_url(client_id, scopes, false, "http://non.existant/");
    println!("## Authorization ##");
    println!("Go to this website:\n{}", url);
    
    println!("Enter the URL that you were redirected to: ");
    let mut redirected_url = String::new();
    io::stdin().read_line(&mut redirected_url).unwrap();

    client.redirected(&redirected_url, &state).await.unwrap();

    save_refresh_token(&client.refresh_token().await.unwrap());
}

async fn get_client(credentials: ClientCredentials, client_id: &String) -> Client {
    if let Some(refresh_token) = load_refresh_token() {
        let client = Client::with_refresh(credentials, refresh_token);
        client
    } else {
        let client = Client::new(credentials);
        prompt_refresh_token(&client, client_id).await;
        client
    }
}

pub async fn publish(songs: &Vec<Song>) {
    dotenv().unwrap();

    let client_id = env::var("CLIENT_ID").expect("Please provide environment variable CLIENT_ID");
    let playlist_id = env::var("PLAYLIST_ID").expect("Environment variable PLAYLIST_ID not set");

    let credentials = ClientCredentials::from_env().expect("CLIENT_ID and CLIENT_SECRET not found.");

    let client = get_client(credentials, &client_id).await;

    let mut playlist = client.playlists().get_playlist(&playlist_id, None).await.unwrap().data;
    clear_playlist(&client, &mut playlist).await;
    import_tracks(&client, &playlist, songs).await;

    save_refresh_token(&client.refresh_token().await.unwrap());
}

async fn get_track_id<'a>(client: &Client, name: &String) -> Option<String> {
    let mut items = client.search().search(name, vec![aspotify::ItemType::Track], false, 1, 0, None).await.ok()?.data.tracks?.items;
    items.get(0)?;
    items[0].id.take()
}

async fn import_tracks(client: &Client, playlist: &aspotify::Playlist, songs: &Vec<Song>) {
    // collect tracks
    let total = songs.len();
    let mut tracks: Vec<aspotify::PlaylistItemType<String, String>> = vec![];
    println!("searching all {} tracks", &total);
    for (i, song) in songs.iter().enumerate() {
        if let Some(track_id) = get_track_id(client, &song.title).await {
            println!("{}/{} - found track {} with id {}", &i + 1, &total, &song.title, &track_id);
            tracks.push(aspotify::PlaylistItemType::Track(track_id));
        } else {
            println!("couldn't find {}", &song.title);
        }
    }

    // upload
    println!("uploading all to the playlist");
    add_tracks(client, playlist, tracks).await;
}

async fn add_tracks(client: &Client, playlist: &aspotify::Playlist, tracks: Vec<aspotify::PlaylistItemType<String, String>>) {
    let playlist_id = &playlist.id;
    let mut offset = 0;
    let total = tracks.len();
    while offset < total {
        let batch = (&tracks[offset..min(total, offset + 100)]).to_vec();
        client.playlists().add_to_playlist(playlist_id, batch, None).await.unwrap();
        offset += 100;
    }
}

async fn clear_playlist(client: &Client, playlist: &mut aspotify::Playlist) {
    println!("> clearing playlist...");

    let playlist_id = &playlist.id;
    let mut offset: usize = 0;

    println!("fetching batch {}-{}", offset, offset+50);
    while let Ok(response) = client.playlists().get_playlists_items(playlist_id, 50, offset, None).await {
        // got batch of tracks
        let tracks = response.data.items;
        if tracks.len() == 0 {
            break;
        }

        // remove the batch
        let mut items: Vec<(aspotify::PlaylistItemType<String, String>, Option<&[usize]>)> = vec![];
        for track in tracks {
            if let Some(track) = &track.item {
                if let aspotify::PlaylistItemType::Track(track) = &track {
                    if let Some(track_id) = &track.id {
                        items.push((aspotify::PlaylistItemType::Track(track_id.to_string()), None));
                        continue;
                    }
                }
            }
            println!("debug: is no track {:?}", &track);
        }

        println!("removing batch {}-{}", offset, offset+50);
        playlist.snapshot_id = client.playlists().remove_from_playlist(playlist_id, items, &playlist.snapshot_id).await.unwrap();
        
        offset += 50;
        println!("fetching batch {}-{}", offset, offset+60);
    }
}