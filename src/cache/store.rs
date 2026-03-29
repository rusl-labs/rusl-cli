use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs;

/// The global Content-Addressable Storage (CAS) engine.
pub struct GlobalStore {
    root_dir: PathBuf,
}

impl GlobalStore {
    /// Initializes the immutable CAS store residing in `~/.local/share/rusl/store` (or OS equivalent)
    pub async fn new() -> Result<Self> {
        let root_dir = if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
            proj_dirs.data_dir().join("store")
        } else {
            // Extreme fallback if OS doesn't provide a home directory
            std::env::current_dir()?.join(".rusl").join("store")
        };

        if !root_dir.exists() {
            fs::create_dir_all(&root_dir)
                .await
                .context("Failed to create the global rusl CAS store directory")?;
        }

        Ok(Self { root_dir })
    }

    /// Computes the SHA-256 integrity hash natively
    pub fn compute_integrity(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }

    /// Writes a downloaded schema raw payload directly into immutable global storage.
    /// Returns the absolute `SHA-256` integrity hash and the physical caching filepath.
    pub async fn put(&self, content: &[u8]) -> Result<(String, PathBuf)> {
        let hash = Self::compute_integrity(content);
        let hash_dir = self.root_dir.join(&hash);

        if !hash_dir.exists() {
            fs::create_dir_all(&hash_dir).await?;
        }

        let file_path = hash_dir.join("schema.json");

        // Storage is immutable. If it exists, the content must be identical because it's derived from the hash!
        if !file_path.exists() {
            fs::write(&file_path, content)
                .await
                .with_context(|| format!("Failed to write CAS blob to {:?}", file_path))?;
        }

        Ok((hash, file_path))
    }

    /// Resolves the absolute path of a cached schema strictly by its unique integrity digest.
    #[allow(dead_code)]
    pub fn get_path(&self, integrity_hash: &str) -> Option<PathBuf> {
        let file_path = self.root_dir.join(integrity_hash).join("schema.json");
        if file_path.exists() {
            Some(file_path)
        } else {
            None
        }
    }
}
