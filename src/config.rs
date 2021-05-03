use serde::{Deserialize};

use config::{Config, File};
use std::fs;

#[derive(Debug, Deserialize)]
pub struct PlaylistsStructure {
    pub radio: String,
    pub spotify_playlist_id: String
}

#[derive(Debug, Deserialize)]
pub struct AuthStructure {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct ConfigStructure {
    pub playlists: Vec<PlaylistsStructure>,
    pub auth: AuthStructure
}

pub fn load_config() -> ConfigStructure {
    let mut config_json = Config::default();
    config_json.merge(File::with_name("config.json")).expect("You need to create a config.json. Use config.template.json as a template :)");
    let config = config_json.try_into::<ConfigStructure>().unwrap();
    config
}

pub fn load_refresh_token() -> Option<String> {
    fs::read_to_string(".refresh_token").ok()
}

pub fn save_refresh_token(refresh_token: &String) {
    fs::write(".refresh_token", refresh_token).unwrap();
}