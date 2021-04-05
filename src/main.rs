#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app, window::*};
use futures_channel;
use tokio::{self, sync::mpsc};
use networking::{connect_to_discord, data::GatewayPayload};
use tokio_tungstenite::tungstenite::Message;

#[macro_use]
extern crate bitflags;

mod networking;

#[tokio::main]
async fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Rustcord");
    wind.make_resizable(true);
    wind.end();
    wind.show();

    // Make a mpsc for sending messages and another for receiving messages
    let (write_tx, write_rx) = futures_channel::mpsc::unbounded::<Message>();
    let (read_tx, mut read_rx) = mpsc::channel::<GatewayPayload>(32);

    tokio::spawn(async move {
        loop {
            let message = read_rx.recv().await;
            if let Some(data) = message {
                println!("[main: receive loop] Received {:?}", data);
            }
        }
    });

    tokio::spawn(connect(write_rx, read_tx.clone()));
    app.run().unwrap();
}

async fn connect(write_rx: futures_channel::mpsc::UnboundedReceiver<Message>, read_tx: mpsc::Sender<GatewayPayload>) {
    connect_to_discord(write_rx, read_tx).await;
}