// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{fs, path::PathBuf};

use aquadoggo::LockFile;
use tauri::AppHandle;

use crate::consts::{RESOURCES_DIR, SCHEMA_LOCK_FILE};

pub fn load_schema_lock(app: &AppHandle) -> anyhow::Result<LockFile> {
    let schema_lock_path = app
        .path_resolver()
        .resolve_resource(PathBuf::new().join(RESOURCES_DIR).join(SCHEMA_LOCK_FILE))
        .expect("failed to resolve resource");
    let data = fs::read_to_string(schema_lock_path)?;
    let lock_file = toml::from_str(&data)?;
    Ok(lock_file)
}
