// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use aquadoggo::{ConfigFile, Configuration};
use figment::providers::{Env, Serialized};
use figment::Figment;
use tauri::AppHandle;

use crate::consts::{AQUADOGGO_CONFIG, BLOBS_DIR, RESOURCES_DIR};

/// Get configuration from 1. .toml file, 2. environment variables (in that order, meaning that
/// later configuration sources take precedence over the earlier ones).
pub fn load_config(app: &AppHandle, app_dir_path: PathBuf) -> Result<Configuration> {
    let default_config_path = app
        .path_resolver()
        .resolve_resource(PathBuf::new().join(RESOURCES_DIR).join(AQUADOGGO_CONFIG))
        .expect("failed to resolve resource");

    // Determine if a config file path was provided or if we should look for it in common locations
    let config_file_path = app_dir_path.join(AQUADOGGO_CONFIG);

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
        app_dir_path.to_str().expect("invalid character in path")
    );

    // Override blobs path based on app data directory path.
    config.blobs_base_path = Some(app_dir_path.join(BLOBS_DIR));

    // Merge the config file with any environment variables.
    let figment = Figment::from(Serialized::defaults(config));
    let config: ConfigFile = figment.merge(Env::raw()).extract()?;
    config.try_into()
}
