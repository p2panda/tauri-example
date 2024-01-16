// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod key_pair;

use std::fs::{self, DirBuilder};
use std::path::PathBuf;

use aquadoggo::Node;
use key_pair::generate_or_load_key_pair;
use tauri::{async_runtime, AppHandle};

use crate::config::load_config;

/// Name of file where node's private key is stored.
const PRIVATE_KEY_FILE: &str = "private-key.txt";

/// Directory where `aquadoggo` will store and serve blobs from.
const BLOBS_DIR: &str = "blobs";

/// Temp folder where app data is persisted in dev mode.
const TMP_APP_DATA_PATH: &str = "./tmp";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Get path to the current app data directory.
///
/// If in dev mode app data is persisted to a temporary folder "./tmp" in the project
/// directory. When not in dev mode app data path is based on tauri defaults and app
/// name defined in our tauri.conf.json file.
fn app_data_dir(app: &AppHandle) -> PathBuf {
    if cfg!(dev) {
        let tmp_data_dir = PathBuf::from(TMP_APP_DATA_PATH);
        if fs::read_dir(&tmp_data_dir).is_err() {
            DirBuilder::new()
                .create(&tmp_data_dir)
                .expect("error creating tmp app data directories");
        }
        tmp_data_dir
    } else {
        app.path_resolver()
            .app_data_dir()
            .expect("error resolving app data dir path")
    }
}

/// Launch node with configuration of persistent storage for SQLite database and blobs.
fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();

    // Get the app data directory path.
    let app_data_dir = app_data_dir(&app_handle);

    // Create a KeyPair or load it from private-key.txt file in app data directory.
    //
    // This key pair is used to identify the node on the network, it is not used for signing
    // any application data.
    let key_pair = generate_or_load_key_pair(app_data_dir.join(PRIVATE_KEY_FILE))
        .expect("error generating or loading node key pair");

    // Load the config from app data directory. If this is the first time the app is
    // being run then the default aquadoggo config file is copied into place and used.
    let mut config = load_config(&app_handle, &app_data_dir)?;

    // Set database url based on app data directory path.
    config.database_url = format!(
        "sqlite:{}/db.sqlite3",
        app_data_dir.to_str().expect("invalid character in path")
    );

    // Set blobs path based on app data directory path.
    config.blobs_base_path = app_data_dir.join(BLOBS_DIR);

    // Create blobs directory incase it doesn't exist.
    DirBuilder::new()
        .recursive(true)
        .create(app_data_dir.join(BLOBS_DIR))
        .expect("error creating app data directories");

    // Spawn aquadoggo in own async task.
    async_runtime::spawn(async {
        let node = Node::start(key_pair, config).await;
        node.on_exit().await;
        node.shutdown().await;
    });

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
