use fltk::{app, window::*, frame::*, };
use web_view::*;

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1000, 500, "Rustcord");
    let mut _servers = Frame::default().with_size(50, 500).with_label("servers").set_frame(FrameType::EngravedBox);
    let mut _top_bar = Frame::default().with_pos(50, 0).with_size(950, 50).with_label("top bar").set_frame(FrameType::EngravedBox);
    let mut _channels = Frame::default().with_pos(50, 50).with_size(200, 400).with_label("channels").set_frame(FrameType::EngravedBox);
    let mut _chat = Frame::default().with_pos(250, 50).with_size(500, 400).with_label("chat").set_frame(FrameType::EngravedBox);
    let mut _info = Frame::default().with_pos(50, 450).with_size(200, 50).with_label("info").set_frame(FrameType::EngravedBox);
    let mut _chat_box = Frame::default().with_pos(250, 450).with_size(500, 50).with_label("chat box").set_frame(FrameType::EngravedBox);
    let mut _members = Frame::default().with_pos(750, 50).with_size(250, 450).with_label("members").set_frame(FrameType::EngravedBox);
    wind.make_resizable(true);
    wind.end();
    wind.show();

    // TODO: find a better bindings lib? (or figure out how to properly inject JS code)
    // TODO: figure out why the js on this page breaks; test if it breaks on an OS other than windows
    let mut _login_wind = web_view::builder()
        .title("Discord Login")
        .content(Content::Url("https://discord.com/login"))
        .size(250, 200)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    app.run().unwrap();
}
