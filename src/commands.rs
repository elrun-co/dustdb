use crate::{db::Database, cluster::Cluster};
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;

pub async fn handle_client(
    stream: TcpStream,
    db: Arc<Database>,
    cluster: Arc<Cluster>,
) -> anyhow::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).await? == 0 {
            break;
        }
        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        let response = handle_command(input, db.clone(), cluster.clone()).await;
        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}

pub async fn handle_command(cmd: &str, db: Arc<Database>, cluster: Arc<Cluster>) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return "-ERR empty command".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "PING" => "+PONG".to_string(),
        "SET" if parts.len() >= 3 => {
            db.set(parts[1], parts[2]);
            "+OK".to_string()
        }
        "GET" if parts.len() == 2 => {
            db.get(parts[1]).unwrap_or_else(|| "(nil)".to_string())
        }
        "DEL" if parts.len() == 2 => {
            if db.del(parts[1]) { ":1".to_string() } else { ":0".to_string() }
        }
        "KEYS" => db.keys().join(", "),
        "CLUSTERJOIN" if parts.len() == 2 => {
            cluster.add_peer(parts[1].to_string()).await;
            "+OK peer added".to_string()
        }
        _ => "-ERR unknown command".to_string(),
    }
}

