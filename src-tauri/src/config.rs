// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{fs, path::PathBuf};

use anyhow::Result;
use aquadoggo::{ConfigFile, Configuration};
use tauri::AppHandle;

const AQUADOGGO_CONFIG: &str = "config.toml";

const RESOURCES_PATH: &str = "resources";

/// Load and validate an `aquadoggo` node configuration from .toml file.
fn load_config_file(config_path: &PathBuf) -> Result<Configuration> {
    let config_str = fs::read_to_string(&config_path)?;
    let node_config: ConfigFile = toml::from_str(&config_str)?;
    node_config.try_into()
}

/// Load config file from app data directory if it exists, is not copy
/// default config.toml into the passed path and load it.
pub fn load_config(
    app: &AppHandle,
    app_data_path: &PathBuf,
) -> Result<Configuration, anyhow::Error> {
    // This is the path where we expect our config file to be.
    let config_path = app_data_path.join(AQUADOGGO_CONFIG);

    // Check if the expected config file exists. If not, this is the first time
    // running the app and we want to copy the default into place.
    if fs::read(&config_path).is_err() {
        let default_config_path = app
            .path_resolver()
            .resolve_resource(PathBuf::new().join(RESOURCES_PATH).join(AQUADOGGO_CONFIG))
            .expect("failed to resolve resource");

        fs::copy(default_config_path, &config_path)?;
    }

    // Now we can load the config file.
    load_config_file(&config_path)
}
