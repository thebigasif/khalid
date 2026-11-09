//! The bundled SQLite is new enough to carry the fix for RUSTSEC-2026-0459
//! (vendor/sqlx-sqlite/KHALID.md).

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Connection};

#[tokio::test]
async fn the_bundled_sqlite_is_3_54_or_newer() {
    let mut connection = SqliteConnectOptions::new()
        .in_memory(true)
        .connect()
        .await
        .unwrap();
    let version: String = sqlx::query_scalar("SELECT sqlite_version()")
        .fetch_one(&mut connection)
        .await
        .unwrap();
    connection.close().await.unwrap();

    let parts: Vec<u32> = version.split('.').map(|p| p.parse().unwrap()).collect();
    assert!(parts.get(..2) >= Some(&[3, 54][..]), "SQLite {version}");
}
