use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{MuteConfig, Snowflake, guild::{GuildFolderInfo, GuildSettingChannelOverride}};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// the user's id
    pub id: Snowflake,
    /// the user's username, not unique across the platform
    pub username: String,
    /// the user's 4-digit discord-tag
    pub discriminator: String,
    /// the user's avatar hash
    pub avatar: Option<String>,
    /// whether the user belongs to an OAuth2 application
    pub bot: Option<bool>,
    /// whether the user is an Official Discord System user (part of the urgent message system)
    pub system: Option<bool>,
    /// whether the user has two factor enabled on their account
    mfa_enabled: Option<bool>,
    /// the user's chosen language option
    locale: Option<String>,
    /// whether the email on this account has been verified
    verified: Option<bool>,
    /// the user's email
    pub email: Option<String>,
    /// the flags on a user's account
    pub flags: Option<UserFlags>,
    /// the type of Nitro subscription on a user's account
    /// 0 is None, 1 is Nitro Classic, 2 is Nitro
    pub premium_type: Option<u8>,
    /// the public flags on a user's account
    pub public_flags: Option<UserFlags>,
    premium: Option<bool>,
    phone: Option<String>,
    nsfw_allowed: Option<bool>,
    mobile: Option<bool>,
    desktop: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    timezone_offset: i32,
    theme: String,
    stream_notifications_enabled: bool,
    pub status: String,
    show_current_game: bool,
    restricted_guilds: Value, // TODO: Discover this
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
    friend_source_flags: Value, // TODO: { "all": true }
    friend_discovery_flags: i32,
    explicit_content_filter: i32,
    enable_tts_command: bool,
    disable_games_tab: bool,
    developer_mode: bool,
    detect_platform_accounts: bool,
    default_guilds_restricted: bool,
    pub custom_status: Option<UserCustomStatusInfo>,
    convert_emoticons: bool,
    contact_sync_enabled: bool,
    animate_stickers: i32,
    animate_emoji: bool,
    allow_accessibility_detection: bool,
    afk_timeout: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGuildSettingEntry {
    version: i32,
    pub suppress_roles: bool,
    pub supress_everyone: bool,
    pub muted: bool,
    mute_config: Option<MuteConfig>,
    mobile_push: bool,
    pub message_notifications: i32,
    pub hide_muted_channels: bool,
    pub guild_id: Option<String>, // TODO
    pub channel_overrides: Vec<GuildSettingChannelOverride>
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
pub struct UserCustomStatusInfo {
    pub text: String,
    pub expires_at: String,
    pub emoji_name: String,
    pub emoji_id: String
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct UserFlags: u32 {
        const NONE = 0;
        const DISCORD_EMPLOYEE = 1 << 0;
        const PARTNERED_SERVER_OWNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
        const SYSTEM = 1 << 12;
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const EARLY_VERIFIED_BOT_DEVELOPER = 1 << 17;
    }
}