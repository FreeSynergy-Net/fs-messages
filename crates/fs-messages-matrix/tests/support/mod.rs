//! An encrypted store in a temporary file, for the store tests. Never an in-memory or plain one: the
//! journal refuses those.
#![allow(dead_code)] // each test file uses a different subset

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use fs_objects::{Cipher, EncryptionKey, OpenOptions, StaticKeyProvider, TursoStore};
use tempfile::TempDir;

pub mod server;

/// Open the encrypted store at `path`.
pub async fn open_at(path: &Path) -> TursoStore {
    let options = OpenOptions {
        key_provider: Some(Arc::new(StaticKeyProvider(EncryptionKey {
            cipher: Cipher::Aes256Gcm,
            key: vec![7; 32],
        }))),
        ..OpenOptions::default()
    };
    TursoStore::open_with(path.to_str().unwrap(), options)
        .await
        .unwrap()
}

/// The folders of the stores a test made. Dropped when the thread of the test ends, also when it panics, and
/// the folder goes with them (a `TempDir` removes itself on drop). The store is handed out as a plain `Arc`, so
/// the folder cannot travel with it; the thread of the test is what holds it.
struct Folders(Vec<TempDir>);

thread_local! {
    static FOLDERS: RefCell<Folders> = const { RefCell::new(Folders(Vec::new())) };
}

/// A new encrypted store; its folder is removed when the thread of the test ends.
pub async fn fresh_store() -> Arc<TursoStore> {
    let (store, dir) = store_and_folder().await;
    FOLDERS.with(|f| f.borrow_mut().0.push(dir));
    store
}

/// A new encrypted store and its folder, for the caller that keeps both.
pub async fn store_and_folder() -> (Arc<TursoStore>, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = open_at(&dir.path().join("store.db")).await;
    (Arc::new(store), dir)
}

/// The store of `name`, kept between calls of the same test (its thread). With `clear`, a new one. The stores and their
/// folders go when the thread of the test ends.
pub async fn named_store(name: &str, clear: bool) -> Arc<TursoStore> {
    if !clear && let Some(store) = NAMED.with(|n| n.borrow().get(name).map(|(s, _)| s.clone())) {
        return store;
    }
    let (store, dir) = store_and_folder().await;
    NAMED.with(|n| n.borrow_mut().insert(name.to_owned(), (store.clone(), dir)));
    store
}

thread_local! {
    static NAMED: RefCell<HashMap<String, (Arc<TursoStore>, TempDir)>> = RefCell::new(HashMap::new());
}

/// The folder of the store this thread made last (for the test that the folder goes).
pub fn last_folder() -> std::path::PathBuf {
    FOLDERS.with(|f| f.borrow().0.last().unwrap().path().to_owned())
}
