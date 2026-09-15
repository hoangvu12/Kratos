//! Disposable client-side history. Keys are engine-scoped; credentials never enter this store.
//! Filesystem writes belong on a background task, not the GPUI event loop.

use std::{
    io,
    io::Write,
    path::{Path, PathBuf},
};

use roboco_doc::SessionMessageEntry;
use roboco_proto::{Chat, Device, Session, Space};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CachedRows {
    pub chats: Vec<Chat>,
    pub spaces: Vec<Space>,
    pub devices: Vec<Device>,
    pub sessions: Vec<Session>,
}

#[derive(Clone)]
pub struct EngineCache {
    root: PathBuf,
}

impl EngineCache {
    pub fn new(data_dir: &Path) -> Self {
        Self {
            root: data_dir.join("engine-cache-v1"),
        }
    }

    fn engine_dir(&self, engine_key: &str) -> PathBuf {
        self.root.join(digest(engine_key))
    }

    pub fn load_rows(&self, engine_key: &str) -> Option<CachedRows> {
        read(&self.engine_dir(engine_key).join("rows.json"))
    }

    pub fn save_rows(&self, engine_key: &str, rows: &CachedRows) -> io::Result<()> {
        write(&self.engine_dir(engine_key).join("rows.json"), rows)
    }

    pub fn load_transcript(
        &self,
        engine_key: &str,
        chat_id: &str,
    ) -> Option<Vec<SessionMessageEntry>> {
        read(&self.transcript_path(engine_key, chat_id))
    }

    pub fn save_transcript(
        &self,
        engine_key: &str,
        chat_id: &str,
        entries: &[SessionMessageEntry],
    ) -> io::Result<()> {
        write(&self.transcript_path(engine_key, chat_id), entries)
    }

    fn transcript_path(&self, engine_key: &str, chat_id: &str) -> PathBuf {
        self.engine_dir(engine_key)
            .join(format!("chat-{}.json", digest(chat_id)))
    }

    pub fn forget_engine(&self, engine_key: &str) -> io::Result<()> {
        match std::fs::remove_dir_all(self.engine_dir(engine_key)) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }
}

fn digest(key: &str) -> String {
    format!("{:x}", Sha256::digest(key.as_bytes()))
}

fn read<T: DeserializeOwned>(path: &Path) -> Option<T> {
    // This is a cache, so a partial/obsolete file is a miss. Engine data remains authoritative.
    serde_json::from_slice(&std::fs::read(path).ok()?).ok()
}

fn write<T: Serialize + ?Sized>(path: &Path, value: &T) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("invalid cache path"))?;
    std::fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    serde_json::to_writer(temporary.as_file_mut(), value)?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}
