use super::{gateway::{GatewayPayload, GatewayPayloadData, IdentifyProperties}};

impl crate::networking::data::gateway::GatewayPayloadData {
    pub fn get_identify_message(token: &String) -> GatewayPayload {
        GatewayPayload {
            op: super::gateway::GatewayOpCodes::Identify,
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
}