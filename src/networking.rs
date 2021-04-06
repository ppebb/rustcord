use std::time::Duration;

use data::{gateway::{GatewayPayload, GatewayPayloadData}};
use futures_channel;
use futures_util::{future, pin_mut, StreamExt};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{self, Message, http::StatusCode}};

pub mod data;

async fn connect_to_websocket(wss_url: &str) -> (WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::http::Response<()>) {
    // Convert the input url from a string to a Url
    let url = url::Url::parse(wss_url).unwrap();

    println!("[connect_to_websocket] Connecting to: {}", url);
    let (ws_stream, response) = connect_async(url).await.expect("Failed to connect");
    println!("[connect_to_websocket] Handshake completed");

    (ws_stream, response)
}

async fn receive_message(message: Result<Message, tungstenite::Error>, read_tx: mpsc::Sender<GatewayPayload>) {
    // Convert the message into text
    let message = message.unwrap();
    let text = message.into_text().unwrap();

    println!("[receive_message] Received a message from the gateway");

    // Try to parse the message
    let data: Result<GatewayPayload, serde_json::Error> = serde_json::from_str(text.as_str());
    match data { // Do stuff with the data from the message
        Ok(payload) => {
            println!("[receive_message] Successfully parsed the payload");
            let r = read_tx.try_send(payload);
            if let Err(e) = r {
                println!("[receive_message] Failed to send the data to read_tx: {:?}", e);
            }
            // TODO: Do stuff with the payload
        },
        Err(error) => println!("[receive_message] ERROR!\n\t--> Failed to parse the payload: {}\n\t--> Input is: {}", error, text)
    }
}

pub async fn send_identify(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    tokio::time::sleep(Duration::new(1, 0)).await;
    let payload_data = GatewayPayloadData::get_identify_message(&"No stealy".to_string());
    let data = serde_json::to_string(&payload_data).unwrap();
    tx.unbounded_send(Message::text(data)).unwrap();
    println!("[send_identify] Sent identify message");
}

/// `write_rx` receives messages to be sent to the websocket.
/// Messages received from the websocket are sent to `read_tx` 
pub async fn connect_to_discord(write_rx: futures_channel::mpsc::UnboundedReceiver<Message>, read_tx: mpsc::Sender<GatewayPayload>) {
    let (ws_stream, response) = connect_to_websocket("wss://gateway.discord.gg/?encoding=json&v=8").await;
    if response.status() != StatusCode::SWITCHING_PROTOCOLS {
        println!("[connect_to_discord] Failed to connect to the websocket! {:?}", response);
        return;
    }

    // Split the websocket stream into a write sink and a read stream
    let (write, read) = ws_stream.split();

    // Send every message from the read stream to the receive_message method
    let read_thread = read.for_each(|message| async {
        receive_message(message, read_tx.clone()).await;
    });

    // Send every message from the UnboundedReceiver to the write sink
    let write_thread = write_rx.map(Ok).forward(write);

    // Pin the values on the stack
    pin_mut!(read_thread, write_thread);
    // wait for any of the threads to finish
    future::select(read_thread, write_thread).await;
    println!("[connect_to_discord] Websocket has been closed");
}
