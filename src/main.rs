#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use fltk::{app, window::*};
use futures_channel;
use tokio::{self, sync::mpsc};
use networking::{connect_to_discord, data::gateway::{GatewayOpCodes, GatewayPayload, GatewayPayloadData}, send_identify, send_message, start_heartbeat};
use tokio_tungstenite::tungstenite::Message;

#[macro_use]
extern crate bitflags;

mod networking;

#[tokio::main]
async fn main() {
    let token = String::from("Hidden from git");
    
    // Build the client with default headers containing the auth token
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(token.as_str()).unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap();

    let client = Arc::new(client);

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Rustcord");
    wind.make_resizable(true);
    wind.end();
    wind.show();

    // Make a mpsc for sending messages and another for receiving messages
    let (write_tx, write_rx) = futures_channel::mpsc::unbounded::<Message>();
    let (read_tx, mut read_rx) = mpsc::channel::<GatewayPayload>(32);

    let write_tx_clone = write_tx.clone();
    tokio::spawn(async move {
        loop {
            let client = Arc::clone(&client);
            let message = read_rx.recv().await;
            if let Some(payload) = message {
                println!("[main: receive loop] Received {:?}", payload);
                match payload.op {
                    GatewayOpCodes::Hello => {
                        if let Some(data) = payload.d {
                            if let GatewayPayloadData::HelloData { heartbeat_interval, .. } = data {
                                tokio::spawn(start_heartbeat(heartbeat_interval, write_tx_clone.clone()));
                            }
                        }
                    },
                    GatewayOpCodes::Dispatch => {
                        if let Some(data) = payload.d {
                            match data {
                                GatewayPayloadData::PresenceUpdateData {activities, ..} => {
                                    // Send a message containing the activities of the user when their presence is updated
                                    tokio::spawn(async move {
                                        send_message(&client, format!("{:?}", activities.first().unwrap().name), "829119138475671602".to_string()).await;
                                    });
                                },
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    tokio::spawn(send_identify(token.clone(), write_tx.clone()));

    tokio::spawn(connect(write_rx, read_tx.clone()));
    app.run().unwrap();
}

async fn connect(write_rx: futures_channel::mpsc::UnboundedReceiver<Message>, read_tx: mpsc::Sender<GatewayPayload>) {
    connect_to_discord(write_rx, read_tx).await;
}