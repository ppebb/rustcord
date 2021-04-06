use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

use self::activity::ActivityInfo;
use super::{GuildHashInfo, MuteConfig, Snowflake, channel::ChannelInfo, user::UserInfo};

pub mod activity;

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildInfo {
    /// guild id
    pub id: Snowflake,
    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,
    /// icon hash
    pub icon: Option<String>,
    /// icon hash, returned when in the template object
    pub icon_hash: Option<String>,
    /// splash hash
    pub splash: Option<String>,
    /// discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    discovery_splash: Option<String>,
    /// true if the user is the owner of the guild
    pub owner: Option<bool>,
    /// id of owner
    pub owner_id: String,
    /// total permissions for the user in the guild (excludes overrides)
    permissions: Option<String>,
    /// voice region id for the guild
    pub region: String,
    /// id of afk channel
    afk_channel_id: Option<String>,
    /// afk timeout in seconds
    afk_timeout: i32,
    /// true if the server widget is enabled
    widget_enabled: Option<bool>,
    /// the channel id that the widget will generate an invite to, or null if set to no invite
    widget_channel_id: Option<String>,
    /// verification level required for the guild
    verification_level: GuildVerificationLevel,
    /// default message notifications level <br/>
    /// `ALL_MESSAGES` = 0, `ONLY_MENTIONS` = 1
    default_message_notifications: u8,
    /// explicit content filter level <br/>
    /// `DISABLED` = 0, `MEMBERS_WITHOUT_ROLES` = 1, `ALL_MEMBERS` = 2
    explicit_content_filter: u8,
    /// roles in the guild
    pub roles: Vec<GuildRoleInfo>,
    /// custom guild emojis
    pub emojis: Vec<GuildEmojiInfo>,
    /// enabled [guild features](https://discord.com/developers/docs/resources/guild#guild-object-guild-features)
    pub features: Vec<String>,
    /// required [MFA level](https://discord.com/developers/docs/resources/guild#guild-object-mfa-level) for the guild <br/>
    /// `NONE` = 0, `ELEVATED` = 1
    pub mfa_level: u8,
    /// application id of the guild creator if it is bot-created
    pub application_id: Option<String>,
    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<String>,
    /// [system channel flags](https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags)
    pub system_channel_flags: u8,
    /// the id of the channel where Community guilds can display rules and/or guidelines
    pub rules_channel_id: Option<String>,
    /// when this guild was joined at
    pub joined_at: Option<String>,
    /// true if this is considered a large guild
    pub large: Option<bool>,
    /// true if this guild is unavailable due to an outage
    pub unavailable: Option<bool>,
    /// total number of members in this guild
    pub member_count: Option<i32>,
    /// states of members currently in voice channels; lacks the `guild_id` key
    pub voice_states: Option<Vec<VoiceState>>,
    /// users in the guild
    pub members: Option<Vec<GuildMemberInfo>>,
    /// channels in the guild
    pub channels: Option<Vec<ChannelInfo>>,
    /// presences of the members in the guild, will only include non-offline members if the size is greater than `large threshold`
    pub presences: Option<Vec<PresenceUpdateEventInfo>>,
    /// the maximum number of presences for the guild (the default value, currently 25000, is in effect when null is returned)
    pub max_presences: Option<i32>,
    /// the maximum number of members for the guild
    pub max_members: Option<i32>,
    /// the vanity url code for the guild
    pub vanity_url_code: Option<String>,
    /// the description for the guild, if the guild is discoverable
    pub description: Option<String>,
    /// banner hash
    pub banner: Option<String>,
    /// [premium tier](https://discord.com/developers/docs/resources/guild#guild-object-premium-tier) (Server Boost level)
    pub premium_tier: Option<u8>,
    /// the number of boosts this guild currently has
    pub premium_subscription_count: Option<i32>,
    /// the preferred locale of a Community guild; used in server discovery and notices from Discord; defaults to "en-US"
    pub preferred_locale: String,
    /// the id of the channel where admins and moderators of Community guilds receive notices from Discord
    pub public_updates_channel_id: Option<Snowflake>,
    /// the maximum amount of users in a video channel
    pub max_video_channel_users: Option<i32>,
    /// approximate number of members in this guild, returned from the `GET /guilds/<id>` endpoint when `with_counts` is `true`
    pub approximate_member_count: Option<i32>,
    /// approximate number of non-offline members in this guild, returned from the `GET /guilds/<id>` endpoint when `with_counts` is `true`
    pub approximate_presence_count: Option<i32>,
    /// the welcome screen of a Community guild, shown to new members, returned when in the invite object
    pub welcome_screen: Option<WelcomeScreenInfo>,
    /// Isn't documented but exists in the gateway's ready message
    lazy: Option<bool>,
    /// Isn't documented but exists in the gateway's ready message
    guild_hashes: Option<GuildHashInfo>,
    /// Isn't documented but exists in the gateway's ready message
    threads: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildRoleInfo {
    /// role id
    pub id: Option<String>,
    /// role name
    pub name: String,
    /// integer representation of hexadecimal color code
    pub color: i32,
    /// if this role is pinned in the user listing
    hoist: bool,
    /// position of this role
    pub position: i32,
    /// permission bit set
    pub permissions: String,
    /// whether this role is managed by an integration
    managed: bool,
    /// whether this role is mentionable
    pub mentionable: bool,
    /// the tags this role has
    tags: Option<Value>
}

/// https://discord.com/developers/docs/resources/guild#guild-member-object
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMemberInfo {
    /// the user this guild member represents
    pub user: Option<UserInfo>,
    /// this users guild nickname
    pub nick: Option<String>,
    /// array of [role](https://discord.com/developers/docs/topics/permissions#role-object) object ids
    pub roles: Vec<Snowflake>,
    /// when the user joined the guild
    pub joined_at: String,
    /// when the user started boosting the guild
    pub premium_since: Option<String>,
    /// whether the user is deafened in voice channels
    pub deaf: bool,
    /// whether the user is muted in voice channels
    pub mute: bool,
    /// whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,
    /// total permissions of the member in the channel, including overrides, returned when in the interaction object
    pub permission: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildEmojiInfo {
    /// emoji id
    pub id: Option<Snowflake>,
    /// emoji name
    pub name: Option<String>,
    /// roles this emoji is whitelisted to
    pub roles: Option<Vec<String>>,
    /// user that created this emoji
    pub user: Option<UserInfo>,
    /// whether this emoji must be wrapped in colons
    pub require_colons: Option<bool>,
    /// whether this emoji is managed
    pub managed: Option<bool>,
    /// whether this emoji is animated
    pub animated: Option<bool>,
    /// whether this emoji can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>
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
pub struct GuildFolderInfo {
    pub name: String, 
    pub id: String, // TODO: Find out if it's actually a string
    pub guild_ids: Vec<String>,
    pub color: i32
}

/// https://discord.com/developers/docs/topics/gateway#presence-update
#[derive(Debug, Serialize, Deserialize)]
pub struct PresenceUpdateEventInfo {
    /// the user presence is being updated for
    pub user: Option<UserInfo>,
    /// id of the guild
    pub guild_id: Snowflake,
    /// either "idle", "dnd", "online", or "offline"
    pub status: Option<String>,
    /// user's current activities
    pub activities: Option<Vec<ActivityInfo>>,
    /// user's platform-dependent status
    pub client_status: Option<ClientStatusInfo>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeScreenInfo {
    /// the server description shown in the welcome screen
    pub description: Option<String>,
    /// the channels shown in the welcome screen, up to 5
    pub welcome_channels: Vec<WelcomeScreenChannelInfo>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeScreenChannelInfo {
    /// the channel's id
    pub channel_id: Snowflake,
    /// the description shown for the channel
    pub description: String,
    /// the emoji id, if the emoji is custom
    pub emoji_id: Option<Snowflake>,
    /// the emoji name if custom, the unicode character if standard, or `null` if no emoji is set
    pub emoji_name: Option<String>,
}

/// https://discord.com/developers/docs/topics/gateway#client-status-object
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientStatusInfo {
    /// the user's status set for an active desktop (Windows, Linux, Mac) application session
    pub desktop: Option<String>,
    /// the user's status set for an active mobile (iOS, Android) application session
    pub mobile: Option<String>,
    /// the user's status set for an active web (browser, bot account) application session
    pub web: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceState {
    /// the guild id this voice state is for
    pub guild_id: Option<String>,
    /// the channel id this user is connected to
    pub channel_id: Option<String>,
    /// the user id this voice state is for
    pub user_id: String,
    /// the guild member this voice state is for
    pub member: Option<GuildMemberInfo>,
    /// the session id for this voice state
    pub session_id: String,
    /// whether this user is deafened by the server
    pub deaf: bool,
    /// whether this user is muted by the server
    pub mute: bool,
    /// whether this user is locally deafened
    pub self_deaf: bool,
    /// whether this user is locally muted
    pub self_mute: bool,
    /// whether this user is streaming using "Go Live"
    pub self_stream: Option<bool>,
    /// whether this user's camera is enabled
    pub self_video: bool,
    /// whether this user is muted by the current user
    pub suppress: bool
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum GuildVerificationLevel {
    /// unrestricted
    None = 0,
    /// must have verified email on account
    Low = 1,
    /// must be registered on Discord for longer than 5 minutes
    Medium = 2,
    /// must be a member of the server for longer than 10 minutes
    High = 3,
    /// must have a verified phone number
    VeryHigh = 4
}