use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

use super::{ConnectedAccountInfo, PayloadEntryList, ReadStateEntry, channel::PrivateChannelEntry, guild::GuildInfo, user::{UserGuildSettingEntry, UserInfo, UserRelationship, UserSettings}};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayPayload {
    pub op: GatewayOpCodes,
    pub d: Option<GatewayPayloadData>,
    pub s: Option<i32>,
    pub t: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatewayPayloadData {
    HelloData { heartbeat_interval: u32, _trace: Value },
    ReadyData {
        v: u8,
        users: Vec<UserInfo>,
        user_settings: UserSettings,
        user_guild_settings: PayloadEntryList<UserGuildSettingEntry>,
        user: UserInfo,
        tutorial: Value, // TODO
        session_id: String,
        relationships: Vec<UserRelationship>,
        read_state: PayloadEntryList<ReadStateEntry>,
        private_channels: Vec<PrivateChannelEntry>,
        merged_members: Value, // TODO: Convert this thing to a struct
        guilds: Vec<GuildInfo>,
        guild_join_requests: Value, // TODO
        guild_experiments: Value,   // TODO
        geo_ordered_rtc_regions: Vec<String>,
        friend_suggestion_count: Option<i32>,
        experiments: Value, // TODO
        country_code: Option<String>,
        consents: Value, // TODO
        connected_accounts: Vec<ConnectedAccountInfo>,
        analytics_token: String,
        _trace: Value // TODO
    },
    IdentifyMessageData {
        token: String,
        capabilities: i32,
        properties: IdentifyProperties
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub browser_user_agent: String
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum GatewayOpCodes {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11
}