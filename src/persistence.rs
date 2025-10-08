use crate::db::Database;
use std::fs;
use std::io::{Read, Write};
use std::sync::Arc;
use anyhow::Result;
use tokio::time::{self, Duration};

pub struct Persistence {
    file_path: String,
}

impl Persistence {
    pub fn new(path: &str) -> Self {
        Self { file_path: path.to_string() }
    }

    pub fn load_into(&self, db: &Database) -> Result<()> {
        if let Ok(mut f) = fs::File::open(&self.file_path) {
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            db.load(&buf);
        }
        Ok(())
    }

    pub fn save_from(&self, db: &Database) -> Result<()> {
        let mut f = fs::File::create(&self.file_path)?;
        f.write_all(&db.dump())?;
        Ok(())
    }

    pub fn spawn_autosave(self: Arc<Self>, db: Arc<Database>) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = self.save_from(&db) {
                    eprintln!("⚠️ Save error: {e}");
                }
            }
        });
    }
}

