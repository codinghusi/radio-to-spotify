use crate::Song;
use aspotify::{Client, ClientCredentials, Scope};
use std::io;
use std::cmp::min;
use crate::config;


async fn prompt_refresh_token(client: &Client, client_id: &String) {
    let scopes = vec![
        Scope::PlaylistModifyPublic,
        Scope::PlaylistModifyPrivate,
        Scope::UserFollowModify,
    ];
    let (url, state) = aspotify::authorization_url(client_id, scopes, false, "http://non.existant/");
    println!("## Authorization ##");
    println!("Go to this website:\n{}", url);
    
    println!("Enter the URL that you were redirected to: ");
    let mut redirected_url = String::new();
    io::stdin().read_line(&mut redirected_url).unwrap();

    client.redirected(&redirected_url, &state).await.unwrap();

    config::save_refresh_token(&client.refresh_token().await.unwrap());
}

pub async fn get_client(credentials: ClientCredentials, client_id: &String) -> Client {
    if let Some(refresh_token) = config::load_refresh_token() {
        let client = Client::with_refresh(credentials, refresh_token);
        client
    } else {
        let client = Client::new(credentials);
        prompt_refresh_token(&client, client_id).await;
        client
    }
}

pub async fn publish(client: &Client, playlist_id: &str, songs: &Vec<Song>) {
    println!("> fetching the actual playlist");
    let mut playlist = client.playlists().get_playlist(&playlist_id, None).await.unwrap().data;

    println!("> clearing it first");
    clear_playlist(&client, &mut playlist).await;

    println!("> importing...");
    import_tracks(&client, &playlist, songs).await;

    config::save_refresh_token(&client.refresh_token().await.unwrap());
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
    println!("=> searching all {} tracks", &total);
    for (i, song) in songs.iter().enumerate() {
        if let Some(track_id) = get_track_id(client, &song.title).await {
            println!("-> {}/{} - found track {} with id {}", &i + 1, &total, &song.title, &track_id);
            tracks.push(aspotify::PlaylistItemType::Track(track_id));
        } else {
            println!("-> couldn't find {}", &song.title);
        }
    }

    // upload
    println!("=> uploading all to the playlist");
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
    let items: Vec<aspotify::PlaylistItemType<String, String>> = vec![];
    client.playlists().replace_playlists_items(&playlist.id, items).await.unwrap();
}