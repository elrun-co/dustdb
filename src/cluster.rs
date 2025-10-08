use std::sync::Arc;
use tokio::time::{self, Duration};
use crate::db::Database;

#[derive(Clone)]
pub struct Cluster {
    db: Arc<Database>,
    peers: Arc<tokio::sync::RwLock<Vec<String>>>,
}

impl Cluster {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            peers: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    pub async fn add_peer(&self, addr: String) {
        let mut peers = self.peers.write().await;
        if !peers.contains(&addr) {
            peers.push(addr);
        }
    }

    pub async fn list_peers(&self) -> Vec<String> {
        self.peers.read().await.clone()
    }

    pub fn spawn_sync_task(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                let data = self.db.dump();
                let peers = self.peers.read().await.clone();

                for peer in peers {
                    let data = data.clone();
                    tokio::spawn(async move {
                        let _ = tokio::net::TcpStream::connect(peer).await;
                        // In real cluster, send binary dump or diff
                    });
                }
            }
        });
    }
}

