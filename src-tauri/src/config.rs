// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::{self, DirBuilder};
use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::Result;
use aquadoggo::{ConfigFile, Configuration};
use figment::providers::{Env, Serialized};
use figment::Figment;
use tauri::AppHandle;
use tempdir::TempDir;

use crate::consts::{AQUADOGGO_CONFIG, BLOBS_DIR, RESOURCES_DIR};

static TMP_DIR: OnceLock<TempDir> = OnceLock::new();

/// Get path to the current app data directory.
///
/// If in dev mode app data is persisted to an ephemeral tmp folder. Otherwise app data path is
/// based on tauri defaults and app name defined in our tauri.conf.json file.
pub fn app_data_dir(app: &AppHandle) -> Result<PathBuf, anyhow::Error> {
    let path = if cfg!(dev) {
        TMP_DIR
            .get_or_init(|| {
                // Initialise a `TempDir` instance globally to make sure it does not run out of
                // scope and gets deleted before the end of the application runtime
                TempDir::new("p2panda-tauri-example").expect("Could not create temporary directory")
            })
            .path()
            .to_path_buf()
    } else {
        app.path_resolver()
            .app_data_dir()
            .expect("error resolving app data dir")
    };

    Ok(path)
}

/// Get configuration from 1. .toml file, 2. environment variables (in that order, meaning that
/// later configuration sources take precedence over the earlier ones).
pub fn load_config(app: &AppHandle) -> Result<Configuration> {
    let app_data_dir = app_data_dir(&app)?;

    let default_config_path = app
        .path_resolver()
        .resolve_resource(PathBuf::new().join(RESOURCES_DIR).join(AQUADOGGO_CONFIG))
        .expect("failed to resolve resource");

    // Determine if a config file path was provided or if we should look for it in common locations
    let config_file_path = app_data_dir.join(AQUADOGGO_CONFIG);

    // Check if the expected config file exists. If not, this is the first time
    // running the app and we want to copy the default into place.
    if !config_file_path.exists() {
        fs::copy(default_config_path, &config_file_path)?;
    };

    let config_str = fs::read_to_string(config_file_path)?;
    let mut config: ConfigFile = toml::from_str(&config_str)?;

    // Override database url based on app data directory path.
    config.database_url = format!(
        "sqlite:{}/db.sqlite3",
        app_data_dir.to_str().expect("invalid character in path")
    );

    // Override blobs path based on app data directory path.
    config.blobs_base_path = Some(app_data_dir.join(BLOBS_DIR));

    // Create blobs directory incase it doesn't exist.
    DirBuilder::new()
        .recursive(true)
        .create(app_data_dir.join(BLOBS_DIR))?;

    // Merge the config file with any environment variables.
    let figment = Figment::from(Serialized::defaults(config));
    let config: ConfigFile = figment.merge(Env::raw()).extract()?;
    config.try_into()
}
