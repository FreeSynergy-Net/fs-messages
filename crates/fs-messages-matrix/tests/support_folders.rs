//! The folders of the test stores are removed: a store made on a thread leaves nothing when the thread ends.

mod support;

#[test]
fn the_folder_of_a_store_goes_when_the_thread_of_the_test_ends() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let _store = support::fresh_store().await;
            // the folder of the store is the one that holds `store.db`
            tx.send(support::last_folder()).unwrap();
        });
    })
    .join()
    .unwrap();
    let folder = rx.recv().unwrap();
    assert!(!folder.exists(), "{} was left behind", folder.display());
}

#[tokio::test]
async fn the_folder_of_a_kept_pair_goes_with_its_drop() {
    let (store, dir) = support::store_and_folder().await;
    let path = dir.path().to_owned();
    assert!(path.exists());
    drop(store);
    drop(dir);
    assert!(!path.exists());
}
