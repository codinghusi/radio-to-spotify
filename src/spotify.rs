use crate::Song;
use aspotify::{Client, ClientCredentials, Scope, Track, Playlist};
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
    let playlist_name = env::var("PLAYLIST_NAME").expect("Environment variable PLAYLIST_NAME not set");

    let credentials = ClientCredentials::from_env().expect("CLIENT_ID and CLIENT_SECRET not found.");

    let client = get_client(credentials, &client_id).await;

    // Gets the album "Favourite Worst Nightmare" from Spotify, with no specified market.
    let playlist = get_cleared_playlist(&client, &playlist_name).await;

    import_tracks(&client, &playlist.id, songs).await;

    println!("{}, {}", playlist.name, playlist.description.unwrap());

    save_refresh_token(&client.refresh_token().await.unwrap());
}

async fn get_track_id<'a>(client: &Client, name: &String) -> Option<String> {
    let mut items = client.search().search(name, vec![aspotify::ItemType::Track], false, 1, 0, None).await.ok()?.data.tracks?.items;
    items.get(0)?;
    items[0].id.take()
}

async fn import_tracks(client: &Client, playlist_id: &String, songs: &Vec<Song>) {
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
    add_tracks(client, playlist_id, tracks).await;
}

async fn add_tracks(client: &Client, playlist_id: &String, tracks: Vec<aspotify::PlaylistItemType<String, String>>) {
    let mut offset = 0;
    let total = tracks.len();
    while offset < total {
        let batch = (&tracks[offset..min(total, offset + 100)]).to_vec();
        client.playlists().add_to_playlist(playlist_id, batch, None).await.unwrap();
        offset += 100;
    }
}

async fn get_playlist_by_name(client: &Client, name: &String) -> Option<aspotify::PlaylistSimplified> {
    let playlists = client.playlists().current_users_playlists(50, 0).await.ok()?;
    let playlists = playlists.data.items;
    for playlist in playlists {
        if playlist.name.eq(name) {
            return Some(playlist);
        }
    }
    None
}

async fn clear_playlist(client: &Client, playlist: aspotify::PlaylistSimplified) -> Option<aspotify::Playlist> {
    let name = &playlist.name;
    // println!("now I would actually delete playlist {} with id {}", name, &playlist.id);
    client.follow().unfollow_playlist(&playlist.id).await.expect("While clearing playlist: couldn't unfollow");
    let new_playlist = create_playlist(&client, &name).await;
    Some(new_playlist)
}

async fn create_playlist(client: &Client, name: &String) -> aspotify::Playlist {
    client.playlists()
        .create_playlist(&name, true, false, &"".to_string())
        .await
        .expect("While clearing playlist: couldn't create new playlist")
        .data
}

async fn get_cleared_playlist(client: &Client, name: &String) -> aspotify::Playlist {
    if let Some(playlist) = get_playlist_by_name(client, name).await {
        return clear_playlist(client, playlist).await.unwrap();
    }
    create_playlist(&client, &name).await
}