use teloxide::{
    dispatching::dialogue::{Serializer, Storage},
    types::ChatId,
};
use teloxide_rocksdb::{RocksDbStorage, RocksDbStorageError};

use std::{
    fmt::{Debug, Display},
    fs,
    sync::Arc,
};

#[tokio::test(flavor = "multi_thread")]
async fn test_rocksdb_json() {
    fs::remove_dir_all("./test_db1").ok();
    fs::create_dir("./test_db1").unwrap();
    let storage = RocksDbStorage::open(
        "./test_db1/test_db1.rocksdb",
        teloxide::dispatching::dialogue::serializer::Json,
        None,
    )
    .await
    .unwrap();
    test_rocksdb(storage).await;
    fs::remove_dir_all("./test_db1").unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rocksdb_bincode() {
    fs::remove_dir_all("./test_db2").ok();
    fs::create_dir("./test_db2").unwrap();
    let storage = RocksDbStorage::open(
        "./test_db2/test_db2.rocksdb",
        teloxide::dispatching::dialogue::serializer::Bincode,
        None,
    )
    .await
    .unwrap();
    test_rocksdb(storage).await;
    fs::remove_dir_all("./test_db2").unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rocksdb_cbor() {
    fs::remove_dir_all("./test_db3").ok();
    fs::create_dir("./test_db3").unwrap();
    let storage = RocksDbStorage::open(
        "./test_db3/test_db3.rocksdb",
        teloxide::dispatching::dialogue::serializer::Cbor,
        None,
    )
    .await
    .unwrap();
    test_rocksdb(storage).await;
    fs::remove_dir_all("./test_db3").unwrap();
}

type Dialogue = String;

async fn test_rocksdb<S>(storage: Arc<RocksDbStorage<S>>)
where
    S: Send + Sync + Serializer<Dialogue> + 'static,
    <S as Serializer<Dialogue>>::Error: Debug + Display,
{
    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(1)).await.unwrap(), None);
    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(11)).await.unwrap(), None);
    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(256)).await.unwrap(), None);

    Arc::clone(&storage).update_dialogue(ChatId(1), "ABC".to_owned()).await.unwrap();
    Arc::clone(&storage).update_dialogue(ChatId(11), "DEF".to_owned()).await.unwrap();
    Arc::clone(&storage).update_dialogue(ChatId(256), "GHI".to_owned()).await.unwrap();

    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(1)).await.unwrap(), Some("ABC".to_owned()));
    assert_eq!(
        Arc::clone(&storage).get_dialogue(ChatId(11)).await.unwrap(),
        Some("DEF".to_owned())
    );
    assert_eq!(
        Arc::clone(&storage).get_dialogue(ChatId(256)).await.unwrap(),
        Some("GHI".to_owned())
    );

    Arc::clone(&storage).remove_dialogue(ChatId(1)).await.unwrap();
    Arc::clone(&storage).remove_dialogue(ChatId(11)).await.unwrap();
    Arc::clone(&storage).remove_dialogue(ChatId(256)).await.unwrap();

    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(1)).await.unwrap(), None);
    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(11)).await.unwrap(), None);
    assert_eq!(Arc::clone(&storage).get_dialogue(ChatId(256)).await.unwrap(), None);

    // Check that a try to remove a non-existing dialogue results in an error.
    assert!(matches!(
        Arc::clone(&storage).remove_dialogue(ChatId(1)).await.unwrap_err(),
        RocksDbStorageError::DialogueNotFound
    ));
}
