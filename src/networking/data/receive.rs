use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

use super::gateway::{GatewayPayload, GatewayOpCodes, GatewayPayloadData};
use super::sendable;
use crate::ui;

mod dispatch;

pub async fn handle_messages(client: reqwest::Client, mut wss_receive_rx: mpsc::Receiver<GatewayPayload>, wss_write_tx: futures_channel::mpsc::UnboundedSender<Message>, ui: ui::RustcordUI) {
    loop {
        let client = client.clone();
        let wss_write_tx = wss_write_tx.clone();
        let ui = ui.clone();

        // Wait until a message is received from the wss receiver
        // If None is returned, stop the message handling loop
        let message = match wss_receive_rx.recv().await {
            Some(m) => m,
            None => {
                info!("All receive senders have been dropped");
                break;
            }
        };

        debug!("Received from websocket -> {:?}", message);

        // Call different methods depending on the opcode of the payload
        match message.op {
            GatewayOpCodes::Hello => {
                handle_hello(message, wss_write_tx).await;
            },
            GatewayOpCodes::Dispatch => {
                handle_dispatch(message, wss_write_tx, client, ui).await;
            }
            _ => {}
        }
    }
}

async fn handle_hello(message: GatewayPayload, wss_write_tx: futures_channel::mpsc::UnboundedSender<Message>) {
    // Get the data from the message
    // If there's no data, print a warn and return
    let data = match message.d {
        Some(d) => d,
        None => {
            warn!("No data in websocket HELLO message");
            return;
        }
    };

    // Get the heartbeat interval if the payload data is a HelloData variant
    // If it's some other variant, print a warn and return
    let heartbeat_interval = match data {
        GatewayPayloadData::HelloData { heartbeat_interval, .. } => heartbeat_interval,
        _ => {
            warn!("Invalid data in websocket hello message");
            return;
        }
    };

    // Start a thread for sending heartbeats
    tokio::spawn(sendable::start_heartbeat(heartbeat_interval, wss_write_tx.clone()));
}

async fn handle_dispatch(message: GatewayPayload, wss_write_tx: futures_channel::mpsc::UnboundedSender<Message>, client: reqwest::Client, ui: ui::RustcordUI) {
    // Get the data from the message
    // If there's no data, print a warn and return
    let data = match message.d {
        Some(d) => d,
        None => {
            warn!("No data in websocket dispatch message");
            return;
        }
    };

    // Match the variant of data (ignore the values) and call a function to handle the variant
    match data {
        GatewayPayloadData::PresenceUpdateData { .. } => dispatch::handle_presence_update(data, client).await,
        GatewayPayloadData::MessageCreateData { .. } => dispatch::handle_message_create(data, ui).await,
        _ => {}
    };
}