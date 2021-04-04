use std::time::Duration;

use data::GatewayPayload;
use futures_util::{future, pin_mut, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod data;

pub async fn connect_to_websocket(wss_url: &str) {
    // Convert the input url from a string to a Url
    let url = url::Url::parse(wss_url).unwrap();

    // Make an unbounded mpsc (multiple producer - single consumer)
    let (writer_tx, writer_rx) = futures_channel::mpsc::unbounded();

    println!("Connecting to: {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Handshake completed");

    let (write, read) = ws_stream.split();

    // Handle received messages
    let ws_to_stdout = {
        read.for_each(|message| async {
            let message = message.unwrap();
            let data = message.into_text().unwrap();
            
            println!("Received some data");
            //println!("\t-> {}", data);
            let payload: GatewayPayload = serde_json::from_str(data.as_str()).unwrap();
            println!("\t-> {:?}", payload);
        })  
    };

    // Spawn a thread to send the IDENTIFY message
    tokio::spawn(send_identify(writer_tx.clone()));
    
    // Send messages from writer_rx to the write sink
    let sender_to_ws = writer_rx.map(Ok).forward(write);

    pin_mut!(sender_to_ws, ws_to_stdout);
    // await the futures and continue when any of them finishes
    future::select(sender_to_ws, ws_to_stdout).await;
    println!("Websocket has been closed");
}

async fn send_identify(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    tokio::time::sleep(Duration::new(1, 0)).await;
    // TODO: Make an identifier struct with a fn default(token: String)
    let data = "Hidden from commit";
    tx.unbounded_send(Message::text(data)).unwrap();
    println!("Sent message");
}

pub async fn connect_to_discord() {
    connect_to_websocket("wss://gateway.discord.gg/?encoding=json&v=8").await;
}
