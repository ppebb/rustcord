#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app, window::*};
use futures_channel;
use tokio::{self, sync::mpsc};
use networking::{connect_to_discord, data::gateway::{GatewayOpCodes, GatewayPayload, GatewayPayloadData}, send_identify, start_heartbeat};
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

    let write_tx_clone = write_tx.clone();
    tokio::spawn(async move {
        loop {
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
                    }
                    _ => {}
                }
            }
        }
    });

    tokio::spawn(send_identify(write_tx.clone()));

    tokio::spawn(connect(write_rx, read_tx.clone()));
    app.run().unwrap();
}

async fn connect(write_rx: futures_channel::mpsc::UnboundedReceiver<Message>, read_tx: mpsc::Sender<GatewayPayload>) {
    connect_to_discord(write_rx, read_tx).await;
}