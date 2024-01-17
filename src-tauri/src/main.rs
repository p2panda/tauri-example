// SPDX-License-Identifier: AGPL-3.0-or-later

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod consts;
mod key_pair;
mod schema;

use std::fs::{self, DirBuilder};
use std::path::PathBuf;
use std::time::Duration;

use aquadoggo::Node;
use consts::{BLOBS_DIR, PRIVATE_KEY_FILE};
use tauri::{async_runtime, AppHandle};

use crate::config::load_config;
use crate::consts::TMP_APP_DATA_DIR;
use crate::key_pair::generate_or_load_key_pair;
use crate::schema::load_schema_lock;

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
fn app_data_dir(app: &AppHandle) -> Result<PathBuf, anyhow::Error> {
    let app_data_path = if cfg!(dev) {
        let tmp_data_dir = PathBuf::from(TMP_APP_DATA_DIR);
        if fs::read_dir(&tmp_data_dir).is_err() {
            DirBuilder::new().create(&tmp_data_dir)?;
        }
        tmp_data_dir
    } else {
        let path = app
            .path_resolver()
            .app_data_dir()
            .expect("error resolving app data dir");
        path
    };

    // And create blobs directory incase it doesn't exist.
    DirBuilder::new()
        .recursive(true)
        .create(app_data_path.join(BLOBS_DIR))?;

    Ok(app_data_path)
}

/// Launch node with configuration of persistent storage for SQLite database and blobs.
fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();

    // Get the app data directory path.
    let app_data_dir = app_data_dir(&app_handle)?;

    // Create a KeyPair or load it from private-key.txt file in app data directory.
    //
    // This key pair is used to identify the node on the network, it is not used for signing
    // any application data.
    let key_pair = generate_or_load_key_pair(app_data_dir.join(PRIVATE_KEY_FILE))?;

    // Load the config from app data directory. If this is the first time the app is
    // being run then the default aquadoggo config file is copied into place and used.
    let config = load_config(&app_handle, &app_data_dir)?;

    // Load the schema.lock file.
    let schema_lock = load_schema_lock(&app_handle)?;

    // Channel for signaling that the node is started.
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn aquadoggo in own async task.
    async_runtime::spawn(async {
        // Start the node.
        let node = Node::start(key_pair, config).await;

        // Migrate the app schemas
        let did_migrate_schemas = node
            .migrate(schema_lock)
            .await
            .expect("failed to migrate app schema");
        if did_migrate_schemas {
            println!("Schema migration: app schemas successfully deployed on initial start-up");
            // Sleep for a second to let the schemas and GraphQL API be built
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        let _ = tx.send(());

        node.on_exit().await;
        node.shutdown().await;
    });

    // Block until the node has started.
    let _ = rx.blocking_recv();

    Ok(())
}

fn main() {
    // Enable logging if set via `RUST_LOG` environment variable.
    if std::env::var("RUST_LOG").is_ok() {
        let _ = env_logger::builder().try_init();
    }

    tauri::Builder::default()
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
