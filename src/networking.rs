use std::time::Duration;

use data::GatewayPayload;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod data;

pub async fn connect_to_websocket(wss_url: &str) {
    let url = url::Url::parse(wss_url).unwrap();

    let (writer_tx, writer_rx) = futures_channel::mpsc::unbounded();

    println!("Connecting to: {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Handshake completed with: {}", wss_url);

    let (write, read) = ws_stream.split();

    let ws_to_stdout = {
        read.for_each(|message| async {
            let message = message.unwrap();
            let data = message.into_text().unwrap();
            // let message_clone = message.clone();
            // let data = message.into_data();
            // tokio::io::stdout().write_all(&data).await.unwrap();
            println!("Received: {}", data);
            // let payload: GatewayPayload = serde_json::from_str(data.as_str()).unwrap();
            // println!("Payload data is: {:?}", payload.d);
        })  
    };

    
    tokio::spawn(send_messages(writer_tx));
    // let sender = write as futures::channel::mpsc::UnboundedSender<Message>;
    // sender.unbounded_send(Message::binary(r#"{"op": 2, "d": {"token": "ODI2ODg2Nzc3NTY5OTM1MzYw.YGTAzg.FaMRY812p7wQcAB3mamMyfjDtSA", "presence": {"status": "online", "since": 0, "activities": [], "afk": false}, "capabilities": 61, "properties": {"os": "Mystery", "browser": "Mystery", "browser_user_agent": "Why do you care"}, "client_state": {"guild_hashes": {}, "highest_last_message_id": "0", "read_state_version": 0, "user_guild_settings_version": -1}}}"#));
    let sender_to_ws = writer_rx.map(Ok).forward(write);

    pin_mut!(sender_to_ws, ws_to_stdout);
    future::select(sender_to_ws, ws_to_stdout).await;
    //ws_to_stdout.await;
    println!("ended");
}

async fn send_messages(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let data = r#"{"op": 2, "d": {"token": "ODI2ODg2Nzc3NTY5OTM1MzYw.YGTAzg.FaMRY812p7wQcAB3mamMyfjDtSA", "presence": {"status": "online", "since": 0, "activities": [], "afk": false}, "capabilities": 61, "properties": {"os": "Mystery", "browser": "Mystery", "browser_user_agent": "Why do you care"}, "client_state": {"guild_hashes": {}, "highest_last_message_id": "0", "read_state_version": 0, "user_guild_settings_version": -1}}}"#;
        print!("Sending... ");
        tx.unbounded_send(Message::text(data)).unwrap();
        println!("{}", data);
    
    loop {
        tokio::time::sleep(Duration::new(5, 0)).await;
    }
}

pub async fn connect_to_discord() {
    connect_to_websocket("wss://gateway.discord.gg/?encoding=json&v=8").await;
    // connect_to_websocket("wss://echo.websocket.org").await;
}
