// SPDX-License-Identifier: AGPL-3.0-or-later

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod consts;
mod key_pair;
mod schema;

use std::time::Duration;

use aquadoggo::Node;
use tauri::{async_runtime, Manager, State};

use crate::config::{app_data_dir, load_config};
use crate::key_pair::generate_or_load_key_pair;
use crate::schema::load_schema_lock;

struct HttpPort(u16);

// Tauri command for passing the http port used by the node to the frontend code.
#[tauri::command]
fn http_port_command(state: State<HttpPort>) -> u16 {
    state.0
}

/// Launch node with configuration of persistent storage for SQLite database and blobs.
fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Get a handle on the running application which gives us access to global state.
    let app = app.handle();
    // Get app data dir path and create the directory if this is the first time the app runs.
    let app_data_dir = app_data_dir(&app)?;

    // Create a KeyPair or load it from private-key.txt file in app data directory.
    //
    // This key pair is used to identify the node on the network, it is not used for signing
    // any application data.
    let key_pair = generate_or_load_key_pair(&app_data_dir)?;

    // Load the config from the app data directory. If this is the first time the app is
    // being run then the default aquadoggo config file is copied into place and used.
    //
    // Environment variables are also parsed and will take priority over values in the config
    // file.
    let config = load_config(&app, &app_data_dir)?;

    // Add the configured nodes http port to the app state so we can access it from the frontend.
    app.manage(HttpPort(config.http_port));

    // Manually construct the app WebView window as we want to set a custom data directory.
    tauri::WindowBuilder::new(&app, "main", tauri::WindowUrl::App("index.html".into()))
        .data_directory(app_data_dir)
        .resizable(false)
        .fullscreen(false)
        .inner_size(800.0, 600.0)
        .title("p2panda-tauri-example")
        .build()?;

    // Load the schema.lock file containing our app schema which will be published to the node.
    let schema_lock = load_schema_lock(&app)?;

    // Channel for signaling that the node is ready.
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn aquadoggo in own async task.
    async_runtime::spawn(async {
        // Start the node.
        let node = Node::start(key_pair, config).await;

        // Migrate the app schemas, returns true if schema were migrated, false if no migration was required.
        let did_migrate_schemas = node
            .migrate(schema_lock)
            .await
            .expect("failed to migrate app schema");

        if did_migrate_schemas {
            println!("Schema migration: app schemas successfully deployed on initial start-up");
            // If schema were migrated it may take some time for the GraphQL API to rebuild.
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        // Signal that schema are migrated so that tauri will progress to launch the app.
        let _ = tx.send(());

        node.on_exit().await;
        node.shutdown().await;
    });

    // Block until schema are migrated.
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
        .invoke_handler(tauri::generate_handler![http_port_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
