use data::GatewayPayload;
use futures::{StreamExt, pin_mut};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

pub mod data;

pub async fn connect_to_websocket(wss_url: &str) {
    let url = url::Url::parse(wss_url).unwrap();

    println!("Connecting to: {}", url);
    let (ws_stream, response) = connect_async(url).await.expect("Failed to connect");
    println!("Handshake completed with: {}", wss_url);

    // println!("Response is: {:?}", response);

    let (write, read) = ws_stream.split();

    let ws_to_stdout = {
        read.for_each(|message| async {
            let message = message.unwrap();
            let data = message.into_text().unwrap();
            // let message_clone = message.clone();
            // let data = message.into_data();
            // tokio::io::stdout().write_all(&data).await.unwrap();
            println!("{}", data);
            let payload: GatewayPayload = serde_json::from_str(data.as_str()).unwrap();
            println!("Payload data is: {:?}", payload.d);
        })  
    };

    pin_mut!(ws_to_stdout);
    ws_to_stdout.await;
}

pub async fn connect_to_discord() {
    connect_to_websocket("wss://gateway.discord.gg/?encoding=json&v=8").await;
}