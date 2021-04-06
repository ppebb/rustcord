use fltk::{app, window::*, frame::*, browser::*, button::*, input::*};

fn main() {
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
    app.run().unwrap();

}
