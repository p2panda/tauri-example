// SPDX-License-Identifier: AGPL-3.0-or-later

/// Name of file where node's private key is stored.
pub(crate) const PRIVATE_KEY_FILE: &str = "private-key.txt";

/// Temp folder where app data is persisted in dev mode.
pub(crate) const TMP_APP_DATA_DIR: &str = "./tmp";

/// Name of `aquadoggo` config toml file.
pub(crate) const AQUADOGGO_CONFIG: &str = "config.toml";

/// Name of directory where tauri resources are located.
pub(crate) const RESOURCES_DIR: &str = "resources";

/// Directory where `aquadoggo` will store and serve blobs from.
pub(crate) const BLOBS_DIR: &str = "blobs";

/// Schema lock file path
pub(crate) const SCHEMA_LOCK_FILE: &str = "schemas/schema.lock";
