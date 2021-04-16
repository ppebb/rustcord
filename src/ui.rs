use fltk::*;

#[derive(Clone)]
pub struct RustcordUI {
    pub app: app::App,
    pub window: window::DoubleWindow,
    pub guilds: frame::Frame,
    pub top_bar: frame::Frame,
    pub channels: frame::Frame,
    pub info: frame::Frame,
    pub members: frame::Frame,
    pub chat_messages: browser::HoldBrowser,
    pub chat_send_button: button::ReturnButton,
    pub chat_text_input: input::Input
}

impl Default for RustcordUI {
    fn default() -> Self {
        let mut ui = RustcordUI {
            app: app::App::default(),
            window: window::Window::new(100, 100, 1000, 500, "Rustcord"),
            guilds: frame::Frame::default().with_size(50, 500).with_label("servers"),
            top_bar: frame::Frame::default().with_pos(50, 0).with_size(950, 50).with_label("top bar"),
            channels: frame::Frame::default().with_pos(50, 50).with_size(200, 400).with_label("channels"),
            info: frame::Frame::default().with_pos(50, 450).with_size(200, 50).with_label("info"),
            members: frame::Frame::default().with_pos(750, 50).with_size(250, 450).with_label("members"),
            chat_messages: browser::HoldBrowser::default().with_size(500, 400).with_pos(250, 50),
            chat_send_button: button::ReturnButton::default().with_pos(700, 450).with_size(50, 50).with_label("send"),
            chat_text_input: input::Input::default().with_pos(250, 450).with_size(450, 50)
        };

        // Set the framing of the different widgets
        ui.guilds.set_frame(FrameType::EngravedBox);
        ui.top_bar.set_frame(FrameType::EngravedBox);
        ui.channels.set_frame(FrameType::EngravedBox);
        ui.info.set_frame(FrameType::EngravedBox);
        ui.members.set_frame(FrameType::EngravedBox);

        ui.window.make_resizable(true);
        ui.window.end();
        ui.window.show();

        // Create a callback for when the button is pressed or enter is pressed
        let mut ui_c = ui.clone();
        ui.chat_send_button.set_callback(move || {
            // Only add the value if the input isn't empty
            if !ui_c.chat_text_input.value().is_empty() {
                ui_c.chat_messages.add(&ui_c.chat_text_input.value());
                ui_c.chat_text_input.set_value("");
            }
        });

        ui
    }
}

impl RustcordUI {
    pub fn new() -> Self {
        Default::default()
    }

    /// Changes the callback of self.chat_send_button to send the content to a discord message.
    /// Will only show the message after the websocket acknowledges it exists
    pub fn set_send_callback_to_discord(&mut self, client: reqwest::Client) {
        let self_c = self.clone();
        self.chat_send_button.set_callback(move || {
            let client = client.clone();
            // Only add the value if the input isn't empty
            if !self_c.chat_text_input.value().is_empty() {
                // Clone the content of the input so there are no race conditions
                let content = self_c.chat_text_input.value().clone();
                tokio::spawn(async move {
                    // TODO: Change the channel id to the selected channel
                    crate::sendable::send_message(client, content, "829119138475671602".to_string()).await;
                });
                self_c.chat_text_input.set_value("");
            }
        });
    }
}