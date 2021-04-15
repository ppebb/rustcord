use crate::networking::data::gateway::GatewayPayloadData;

pub async fn handle_presence_update(data: GatewayPayloadData, client: reqwest::Client) {
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
        super::sendable::send_message(client.clone(), format!("Hi {:?}, your rpc is: {:?}", user.username, activities.first().unwrap().name), "829119138475671602".to_string()).await;
    });
}