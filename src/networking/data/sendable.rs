use std::time::Duration;

use tokio_tungstenite::tungstenite::Message;

use super::{gateway::{GatewayOpCodes, GatewayPayload, GatewayPayloadData, IdentifyProperties}};

pub fn create_identify_message(token: &String) -> GatewayPayload {
    GatewayPayload {
        op: GatewayOpCodes::Identify,
        s: None,
        t: None,
        d: Some(GatewayPayloadData::IdentifyMessageData {
            token: token.to_owned(),
            capabilities: 61,
            properties: IdentifyProperties {
                browser: "Super real browser".to_string(),
                os: "Rusty system".to_string(),
                browser_user_agent: "Agent 007".to_string()
            }
        })
    }
}

pub fn create_heartbeat_message() -> GatewayPayload {
    GatewayPayload {
        op: GatewayOpCodes::Heartbeat,
        s: None,
        t: None,
        d: None
    }
}

pub async fn start_heartbeat(delay: u32, write_tx: futures_channel::mpsc::UnboundedSender<Message>) {
    trace!("Started heartbeat thread");
    let mut interval = tokio::time::interval(Duration::from_millis(delay as u64));
    loop {
        interval.tick().await;
        trace!("Sending heartbeat");
        let heartbeat_json = match serde_json::to_string(&create_heartbeat_message()) {
            Ok(j) => j,
            Err(e) => {
                error!("Failed to create the heartbeat json with error: {}", e);
                return;
            }
        };

        let send_result = write_tx.unbounded_send(Message::text(heartbeat_json));
        if let Err(e) = send_result {
            error!("Failed to send the heartbeat with error: {}", e);
        }
    }
}

pub async fn send_message(client: reqwest::Client, content: String, channel_id: String) {
    let url = format!("https://discord.com/api/v8/channels/{}/messages", channel_id);
    let json = serde_json::json!({
        "content": &content
    });

    // create the post request and send it
    let res = client  
        .post(&url)
        .json(&json)
        .send()
        .await.unwrap();

    trace!("Sent message to {} with response: {:?}", url, res);
}

pub async fn send_identify(token: String, tx: futures_channel::mpsc::UnboundedSender<Message>) {
    tokio::time::sleep(Duration::new(1, 0)).await;
    let payload_data = create_identify_message(&token);
    let data = serde_json::to_string(&payload_data).unwrap();
    tx.unbounded_send(Message::text(data)).unwrap();
    trace!("Sent identify message");
}