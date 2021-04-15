#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app, window::*, frame::*, browser::*, button::*, input::*};
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

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1000, 500, "Rustcord");
    let servers = Frame::default().with_size(50, 500).with_label("servers").set_frame(FrameType::EngravedBox);
    let top_bar = Frame::default().with_pos(50, 0).with_size(950, 50).with_label("top bar").set_frame(FrameType::EngravedBox);
    let channels = Frame::default().with_pos(50, 50).with_size(200, 400).with_label("channels").set_frame(FrameType::EngravedBox);
    let info = Frame::default().with_pos(50, 450).with_size(200, 50).with_label("info").set_frame(FrameType::EngravedBox);
    let members = Frame::default().with_pos(750, 50).with_size(250, 450).with_label("members").set_frame(FrameType::EngravedBox);
    let mut chat_messages = HoldBrowser::default().with_size(500, 400).with_pos(250, 50);
    let mut but1 = ReturnButton::default().with_pos(700, 450).with_size(50, 50).with_label("send");
    let mut chat_input = Input::default().with_pos(250, 450).with_size(450, 50);
    wind.make_resizable(true);
    wind.end();
    wind.show();

    but1.set_callback(move || {
        // Only add the value if the input isn't empty
        if !chat_input.value().is_empty() {
            chat_messages.add(&chat_input.value());
            chat_input.set_value("");
        }
    });
    
    // Make a mpsc for sending messages to discord and another for receiving messages from discord
    let (send_tx, send_rx) = futures_channel::mpsc::unbounded::<Message>();
    let (receive_tx, receive_rx) = mpsc::channel::<GatewayPayload>(32);
    
    tokio::spawn(connect(send_rx, receive_tx.clone())); // Spawn a thread to connect to the websocket
    tokio::spawn(handle_messages(client.clone(), receive_rx, send_tx.clone())); // Spawn a thread to handle the messages received from the websocket
    tokio::spawn(sendable::send_identify(token.clone(), send_tx.clone())); // Spawn a thread to send an identify message to the websocket
    
    app.run().unwrap();
}

async fn connect(write_rx: futures_channel::mpsc::UnboundedReceiver<Message>, read_tx: mpsc::Sender<GatewayPayload>) {
    connect_to_discord(write_rx, read_tx).await;
}