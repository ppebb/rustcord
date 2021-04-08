use serde::{Deserialize, Serialize};
use serde_repr::*;

use super::{Snowflake, user::UserInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateChannelEntry {
    #[serde(rename="type")]
    channel_type: ChannelTypes,
    recipient_ids: Vec<String>,
    last_message_id: String,
    id: String
}

/// https://discord.com/developers/docs/resources/channel
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    /// the id of this channel
    pub id: Snowflake,
    /// the type of channel
    #[serde(rename="type")]
    pub channel_type: ChannelTypes,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
    /// sorting position of the channel
    pub position: Option<i32>,
    /// explicit permission overwrites for members and roles
    pub permission_overwrites: Vec<ChannelPermissionOverwrite>,
    // the name of the channel (2-100 characters)
    pub name: Option<String>,
    /// the channel topic (0-1024 characters)
    pub topic: Option<String>,
    /// whether the channel is nsfw
    pub nsfw: Option<bool>,
    /// the id of the last message sent in this channel (may not point to an existing or valid message)
    pub last_message_id: Option<Snowflake>,
    /// the bitrate (in bits) of the voice channel
    pub bitrate: Option<i32>,
    /// the user limit of the voice channel
    pub user_limit: Option<i32>,
    /// amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission `manage_messages` or `manage_channel`, are unaffected
    pub rate_limit_per_user: Option<i32>,
    /// the recipients of the DM
    pub recipients: Vec<UserInfo>,
    /// icon hash
    pub icon: Option<String>,
    /// id of the DM creator
    pub owner_id: Option<Snowflake>,
    /// application id of the group DM creator if it is bot-created
    pub application_id: Option<Snowflake>,
    /// id of the parent category for a channel (each parent category can contain up to 50 channels)
    pub parent_id: Option<Snowflake>,
    /// when the last pinned message was pinned. This may be null in events such as GUILD_CREATE when a message is not pinned.
    last_pin_timestamp: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelPermissionOverwrite {
    /// role or user id
    id: String,
    /// either 0 (role) or 1 (member)
    #[serde(rename="type")]
    target_type: u8,
    /// permission bit set
    allow: String,
    /// permission bit set
    deny: String
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ChannelTypes {
    /// a text channel within a server
    GuildText = 0,
    /// a direct message between users
    DM = 1,
    /// a voice channel within a server
    GuildVoice = 2,
    /// a direct message between multiple users
    GroupDm = 3,
    /// an organizational category that contains up to 50 channels
    GuildCategory = 4,
    /// a channel that users can follow and crosspost into their own server
    GuildNews = 5,
    /// a channel in which game developers can sell their game on Discord
    GuildStore = 6
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ChannelPermissionFlags: u64 {
        const CREATE_INSTANT_INVITE = 1 << 0;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        const MANAGE_EMOJIS  = 1 << 30;
    }
}