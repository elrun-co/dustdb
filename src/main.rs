mod db;
mod cluster;
mod commands;
mod persistence;
mod pubsub;
mod protocol;

use std::sync::Arc;
use tokio::net::TcpListener;
use anyhow::Result;

use crate::{db::Database, cluster::Cluster, persistence::Persistence};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 DustDB starting...");

    let db = Arc::new(Database::new());
    let persistence = Arc::new(Persistence::new("dustdb_data.bin"));
    let cluster = Arc::new(Cluster::new(db.clone()));

    // Load persisted data
    persistence.load_into(&db)?;
    println!("📦 Data loaded from disk");

    // Start background sync and persistence tasks
    persistence.spawn_autosave(db.clone());
    cluster.clone().spawn_sync_task();

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("🧠 DustDB listening on 127.0.0.1:6379");

    loop {
        let (stream, _) = listener.accept().await?;
        let db = db.clone();
        let cluster = cluster.clone();

        tokio::spawn(async move {
            if let Err(e) = commands::handle_client(stream, db, cluster).await {
                eprintln!("⚠️ Client error: {e}");
            }
        });
    }
}

