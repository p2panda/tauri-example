// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::File;
use std::io::{Read, Write};
#[cfg(target_os = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use anyhow::Result;
use p2panda_rs::identity::KeyPair;

use crate::consts::PRIVATE_KEY_FILE;

/// Returns a new instance of `KeyPair` by either loading the private key from a path or generating
/// a new one and saving it in the file system.
pub fn generate_or_load_key_pair(path: PathBuf) -> Result<KeyPair> {
    let private_key_path = path.join(PRIVATE_KEY_FILE);
    let key_pair = if private_key_path.is_file() {
        load_key_pair_from_file(private_key_path.clone())?
    } else {
        let key_pair = KeyPair::new();
        save_key_pair_to_file(&key_pair, private_key_path.clone())?;
        key_pair
    };

    Ok(key_pair)
}

/// Saves human-readable (hex-encoded) private key string (ed25519) into a file at the given path.
///
/// This method automatically creates the required directories on that path and fixes the
/// permissions of the file (0600, read and write permissions only for the owner).
#[cfg(target_os = "unix")]
pub fn save_key_pair_to_file(key_pair: &KeyPair, path: PathBuf) -> Result<()> {
    let private_key_hex = hex::encode(key_pair.private_key().as_bytes());

    let mut file = File::create(&path)?;
    file.write_all(private_key_hex.as_bytes())?;
    file.sync_all()?;

    // Set permission for sensitive information
    let mut permissions = file.metadata()?.permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)?;

    Ok(())
}

#[cfg(not(target_os = "unix"))]
pub fn save_key_pair_to_file(key_pair: &KeyPair, path: PathBuf) -> Result<()> {
    let private_key_hex = hex::encode(key_pair.private_key().as_bytes());

    let mut file = File::create(path)?;
    file.write_all(private_key_hex.as_bytes())?;
    file.sync_all()?;

    Ok(())
}

/// Loads a private key from a file at the given path and derives ed25519 key pair from it.
///
/// The private key in the file needs to be represented as a hex-encoded string.
fn load_key_pair_from_file(path: PathBuf) -> Result<KeyPair> {
    let mut file = File::open(path)?;
    let mut private_key_hex = String::new();
    file.read_to_string(&mut private_key_hex)?;
    let key_pair = KeyPair::from_private_key_str(&private_key_hex)?;
    Ok(key_pair)
}
