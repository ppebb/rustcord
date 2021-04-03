use serde_json::Value;
use serde_repr::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayPayload {
    pub op: GatewayOpCodes,
    pub d: Option<PayloadData>,
    pub s: Option<i32>,
    pub t: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayloadData {
    HelloData { heartbeat_interval: u32, _trace: Value },
    ReadyData {
        v: u8,
        users: Vec<UserInfo>,
        user_settings: UserSettings,
        user_guild_settings: PayloadEntryList<UserGuildSettingEntry>,
        user: UserInfo,
        tutorial: Option<Value>,
        session_id: String,
        relationships: Vec<UserRelationship>,
        read_state: PayloadEntryList<ReadStateEntry>,
        private_channels: Vec<PrivateChannelEntry>,
        merged_members: Value, // TODO: Convert this thing to a struct
        guilds: Vec<GuildInfo>
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    public_flags: i32,
    pub id: String, // User id
    pub discriminator: String, // Account#3106, the 3106 is the discriminator
    pub avatar: String, // The avatar id of the account
    verified: Option<bool>,
    premium_type: Option<i32>,
    premium: Option<bool>,
    phone: Option<String>,
    nsfw_allowed: Option<bool>,
    mobile: Option<bool>,
    mfa_allowed: Option<bool>,
    flags: Option<i32>,
    pub email: Option<String>,
    desktop: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    timezone_offset: i32,
    theme: String,
    stream_notifications_enabled: bool,
    pub status: String,
    show_current_game: bool,
    restricted_guilds: Vec<Value>, // TODO: Discover this
    render_reactions: bool,
    render_embeds: bool,
    native_phone_integration_enabled: bool,
    message_display_compact: bool,
    locale: String,
    inline_embed_media: bool,
    inline_attachment_media: bool,
    guild_positions: Vec<String>, // How the channels are sorted in the left bar
    guild_folders: Vec<GuildFolderInfo>,
    gif_auto_play: bool,
    friend_source_flags: Vec<Value>, // TODO: { "all": true }
    friend_discovery_flags: i32,
    explicit_content_filter: i32,
    enable_tts_command: bool,
    disable_games_tab: bool,
    developer_mode: bool,
    detect_platform_accounts: bool,
    default_guilds_restricted: bool,
    pub custom_status: Option<CustomStatusInfo>,
    convert_emoticons: bool,
    contact_sync_enabled: bool,
    animate_stickers: i32,
    animate_emoji: bool,
    allow_accessibility_detection: bool,
    afk_timeout: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadEntryList<T> {
    pub version: i32,
    pub partial: bool,
    pub entries: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGuildSettingEntry {
    version: i32,
    pub suppress_roles: bool,
    pub supress_everyone: bool,
    pub muted: bool,
    mute_config: Option<MuteConfig>, // TODO
    mobile_push: bool,
    pub message_notifications: i32,
    pub hide_muted_channels: bool,
    pub guild_id: Option<String>, // TODO
    pub channel_overrides: Vec<GuildSettingChannelOverride>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadStateEntry {
    pub mention_count: i32,
    pub last_pin_timestamp: String,
    pub last_message_id: String,
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildSettingChannelOverride {
    pub muted: bool,
    pub mute_config: MuteConfig,
    pub message_notifications: i32,
    pub collapsed: bool,
    pub channel_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuteConfig {
    pub selected_time_window: i32,
    pub end_time: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildFolderInfo {
    pub name: String, 
    pub id: String, // TODO: Find out if it's actually a string
    pub guild_ids: Vec<String>,
    pub color: String, // TODO: Find out if it's actually a string
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomStatusInfo {
    pub text: String,
    pub expires_at: String,
    pub emoji_name: String,
    pub emoji_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRelationship {
    pub user_id: String,
    #[serde(rename="type")]
    pub relationship_type: u8,
    nickname: Option<String>,
    id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateChannelEntry {
    #[serde(rename="type")]
    channel_type: i32,
    recipient_ids: Vec<String>,
    last_message_id: String,
    id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildInfo {
    description: Option<String>,
    max_members: u32,
    afk_channels_id: Option<String>,
    roles: Vec<RoleInfo>,
    lazy: bool,
    // TODO: Continue with channels inside guilds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: Option<String>,     // role id
    pub name: String,           // role name
    pub color: i32,             // integer representation of hexadecimal color code
    hoist: bool,                // if this role is pinned in the user listing
    pub position: i32,          // position of this role
    pub permissions: String,    // permission bit set
    managed: bool,              // whether this role is managed by an integration
    pub mentionable: bool,      // whether this role is mentionable
    tags: Option<Value>         // the tags this role has
}

bitflags! {
    struct PermissionFlags: u32 {
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
