use fltk::{app, window::*, frame::*, };

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1000, 500, "Rustcord");
    let mut servers = Frame::default().with_size(50, 500).with_label("servers").set_frame(FrameType::EngravedBox);
    let mut top_bar = Frame::default().with_pos(50, 0).with_size(950, 50).with_label("top bar").set_frame(FrameType::EngravedBox);
    let mut channels = Frame::default().with_pos(50, 50).with_size(200, 400).with_label("channels").set_frame(FrameType::EngravedBox);
    let mut chat = Frame::default().with_pos(250, 50).with_size(500, 400).with_label("chat").set_frame(FrameType::EngravedBox);
    let mut info = Frame::default().with_pos(50, 450).with_size(200, 50).with_label("info").set_frame(FrameType::EngravedBox);
    let mut chat_box = Frame::default().with_pos(250, 450).with_size(500, 50).with_label("chat box").set_frame(FrameType::EngravedBox);
    let mut members = Frame::default().with_pos(750, 50).with_size(250, 450).with_label("members").set_frame(FrameType::EngravedBox);
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}
