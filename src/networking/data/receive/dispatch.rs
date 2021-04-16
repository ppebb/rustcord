use fltk::BrowserExt;

use crate::networking::data::gateway::GatewayPayloadData;
use crate::ui;

pub async fn handle_presence_update(data: GatewayPayloadData, client: reqwest::Client) {
    // Get the data from the data argument
    // If it's not a PresenceUpdateData variant, return and print a warn
    let (user, status, last_modified, client_status, activities) = match data {
        GatewayPayloadData::PresenceUpdateData { user, status, last_modified, client_status, activities } 
          => (user, status, last_modified, client_status, activities),
        _ => {
            warn!("Invalid data passed to handle_presence_update");
            return;
        }
    };

    // Send a message containing the activities of the user when their presence is updated
    tokio::spawn(async move {
        super::sendable::send_message(client.clone(), format!("Hi {:?}, your rpc is: `{:?}`", user.username, activities.first()), "829119138475671602".to_string()).await;
    });
}

pub async fn handle_message_create(data: GatewayPayloadData, mut ui: ui::RustcordUI) {
    // Get the data from the data argument
    // If it's not a MessageCreateData, return and print a warn
    let message_data = match data {
        GatewayPayloadData::MessageCreateData { message_data }
          => message_data,
        _ => {
            warn!("Invalid data passed to handle_message_create");
            return;
        }
    };

    // Do some pretty formatting to the message and then add it to the ui
    let username = message_data.author.username.unwrap_or("missing_username".to_string());
    let discriminator = message_data.author.discriminator.unwrap_or("missing_discriminator".to_string());
    let content = format!("<{}#{}>: {}", username, discriminator, message_data.content);
    ui.chat_messages.add(content.as_str());
}