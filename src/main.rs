#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app, window::*};
use tokio;
use networking::{connect_to_discord};

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

    tokio::spawn(async {
        connect_to_discord().await
    });
    
    app.run().unwrap();
}
