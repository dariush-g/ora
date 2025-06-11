use iced::{keyboard::Key, widget::shader::wgpu::hal::auxil::db::broadcom};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Mutex, RwLock, broadcast},
};
use uuid::Uuid;

use crate::{
    connection::ClientConnection,
    logs::ChannelLog,
    message::{GlobalMessage, ServerMessage, dh::generate_sent_key, encrypt::DHUser},
    user::UserAccount,
};

#[derive(Debug, Clone)]
pub struct GlobalServer {
    user_accounts: Arc<RwLock<HashMap<Uuid, UserAccount>>>,
    channels: Arc<RwLock<HashMap<Uuid, (Channel, BigUint)>>>,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    members: Vec<(UserAccount, ClientConnection)>,
    logs: ChannelLog,
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Arc::new(RwLock::new(GlobalServer {
        user_accounts: Arc::new(RwLock::new(HashMap::new())),
        channels: Arc::new(RwLock::new(HashMap::new())),
    }));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, rx) = broadcast::channel::<String>(1000);

    eprintln!("Server listening on 127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Client connected: {}", addr);
        let tx_clone = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(handle_client(stream, server.clone(), rx, tx_clone));
    }

    Ok(())
}

async fn handle_client(
    stream: TcpStream,
    global_server: Arc<RwLock<GlobalServer>>,
    rx: broadcast::Receiver<String>,
    tx: broadcast::Sender<String>,
) -> tokio::io::Result<()> {
    // Split the stream into reader and writer
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let writer = Arc::new(Mutex::new(writer));

    // Clone for the writer task
    let writer_clone = Arc::clone(&writer);
    let global_server_clone = Arc::clone(&global_server);
    let mut rx_clone = rx;

    // Spawn writer task to handle outgoing messages
    tokio::spawn(async move {
        while let Ok(msg) = rx_clone.recv().await {
            if let Ok(global_message) = serde_json::from_str::<GlobalMessage>(&msg) {
                match global_message.content {
                    crate::message::encrypt::MessageType::KeyShare(shared_key) => {
                        let server_dh = DHUser::new();
                        let private_key = server_dh.get_private_key();
                        let sent_key = generate_sent_key(&private_key.clone());

                        // Get write access to global server
                        let gs_writer_guard = global_server_clone.write().await;
                        let mut channels_guard = gs_writer_guard.channels.write().await;

                        let secret = server_dh.get_combined_key(shared_key);

                        if let Some((_, key)) = channels_guard.get_mut(&global_message.channel) {
                            *key = secret;

                            if let Ok(message) = serde_json::to_vec(&sent_key) {
                                let mut writer_guard = writer_clone.lock().await;
                                let _ = writer_guard.write_all(&message).await;
                            }
                        }
                    }
                    crate::message::encrypt::MessageType::EncryptedMessage(encrypted) => {
                        // Handle encrypted messages here
                        // You can add your logic for processing encrypted messages
                        if let Some((_, secret)) = global_server_clone
                            .write()
                            .await
                            .channels
                            .write()
                            .await
                            .get(&global_message.channel)
                        {
                            let dec = encrypted.xor_keystream_decrypt(secret);
                        }
                    }
                }
            }
        }
    });

    // Main reader loop for incoming messages
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                if let Err(e) = tx.send(line.clone()) {
                    eprintln!("Error transmitting message: {e}")
                }

                line.clear();
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }

    Ok(())
}
