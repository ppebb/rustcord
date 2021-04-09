use super::{gateway::{GatewayOpCodes, GatewayPayload, GatewayPayloadData, IdentifyProperties}};

pub fn create_identify_message(token: &String) -> GatewayPayload {
    GatewayPayload {
        op: GatewayOpCodes::Identify,
        s: None,
        t: None,
        d: Some(GatewayPayloadData::IdentifyMessageData {
            token: token.to_owned(),
            capabilities: 61,
            properties: IdentifyProperties {
                browser: "Super real browser".to_string(),
                os: "Rusty system".to_string(),
                browser_user_agent: "Agent 007".to_string()
            }
        })
    }
}

pub fn create_heartbeat_message() -> GatewayPayload {
    GatewayPayload {
        op: GatewayOpCodes::Heartbeat,
        s: None,
        t: None,
        d: None
    }
}

pub fn create_send_message_json(content: &String) -> serde_json::Value {
    serde_json::json!({
        "content": content
    })
}