#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::prelude::BrowserExt;
use futures_channel;
use tokio::{self, sync::mpsc};
use networking::{connect_to_discord, data::{gateway::GatewayPayload, receive::handle_messages, sendable}};
use tokio_tungstenite::tungstenite::Message;

#[macro_use]
extern crate bitflags;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod networking;
mod ui;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = String::from("Hidden from git");
    
    // Build the client with default headers containing the auth token
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(token.as_str()).unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap();

    let mut ui = ui::RustcordUI::new();
    ui.set_send_callback_to_discord(client.clone()); // Makes the send button send a message to discord instead of displaying a message

    // Add a test item to the ui
    let mut b = ui.clone();
    b.chat_messages.add("item");
    
    // Make a mpsc for sending messages to discord and another for receiving messages from discord
    let (send_tx, send_rx) = futures_channel::mpsc::unbounded::<Message>();
    let (receive_tx, receive_rx) = mpsc::channel::<GatewayPayload>(32);
    
    tokio::spawn(connect(send_rx, receive_tx.clone())); // Spawn a thread to connect to the websocket
    tokio::spawn(handle_messages(client.clone(), receive_rx, send_tx.clone(), ui.clone())); // Spawn a thread to handle the messages received from the websocket
    tokio::spawn(sendable::send_identify(token.clone(), send_tx.clone())); // Spawn a thread to send an identify message to the websocket
    
    // Start the ui app
    ui.app.run().unwrap();
}

async fn connect(write_rx: futures_channel::mpsc::UnboundedReceiver<Message>, read_tx: mpsc::Sender<GatewayPayload>) {
    connect_to_discord(write_rx, read_tx).await;
}