use serde_json::Value;
use serde_repr::*;
use serde::{Deserialize, Serialize};

use self::user::UserInfo;

// TODO: Maybe convert ids from Strings to a custom Snowflake type
// TODO: Split into multiple files

pub mod gateway;
pub mod user;
pub mod guild;
pub mod channel;
pub mod sendable;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Snowflake(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadEntryList<T> {
    pub version: i32,
    pub partial: bool,
    pub entries: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadStateEntry {
    pub mention_count: i32,
    pub last_pin_timestamp: String,
    pub last_message_id: String,
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuteConfig {
    pub selected_time_window: i32,
    pub end_time: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildHashInfo {
    version: Option<u8>,
    roles: OmittableHash,
    metadata: OmittableHash,
    channels: OmittableHash
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedAccountInfo {
    pub visibility: u8, 
    pub verified: bool,
    #[serde(rename="type")]
    pub connection_type: String,
    pub show_activity: bool,
    pub revoked: bool,
    pub name: String,
    pub id: String,
    pub friend_sync: bool,
    pub access_token: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct OmittableHash {
    omitted: Option<bool>,
    hash: Option<String>
}

// TODO: Move the things below to some other file

#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    /// The default name of the Webhook
    pub name: String,
    /// The type of the Webhook
    #[serde(rename="type")]
    pub webhook_type: i32,
    /// The channel id this Webhook is for
    pub channel_id: Value, // TODO change to snowflake (:
    /// The guild id  this Webhook is for
    pub guild_id: Option<Value>, // TODO change to snowflake (:
    /// The user object
    pub user: Option<UserInfo>,
    /// The secure token of the webhook (returned for incoming Webhooks)
    pub token: Option<String>,
    /// The bot/OAuth2 applications that created this Webhook
    pub application_id: Option<Value>, // TODO change to snowflake (:
    /// The default avatar of the webhook
    pub avator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWebhook {
    /// The name of the Webhook
    pub name: String,
    /// The image for the default Webhook avatar
    pub avatar: Option<Value>, // TODO change to ImageData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyWebhook {
    /// The default name of the Webhook
    pub name : String,
    /// The image for the default Webhook avatar
    pub avatar: Option<Value>, // TODO change to ImageData
    /// The new channel id this Webhook should be moved to
    pub channel_id: Value, // TODO change to snowflake (:
}
