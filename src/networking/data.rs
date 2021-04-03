use serde_json::Value;
use serde_repr::*;
use serde::{Serialize, Deserialize};

// TODO: Maybe convert ids from Strings to a custom Snowflake type

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
    /// the user's id
    pub id: String,
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
    channel_type: ChannelTypes,
    recipient_ids: Vec<String>,
    last_message_id: String,
    id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildInfo {
    /// guild id
    pub id: String,
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
    verification_level: VerificationLevel,
    /// default message notifications level <br/>
    /// `ALL_MESSAGES` = 0, `ONLY_MENTIONS` = 1
    default_message_notifications: u8,
    /// explicit content filter level <br/>
    /// `DISABLED` = 0, `MEMBERS_WITHOUT_ROLES` = 1, `ALL_MEMBERS` = 2
    explicit_content_filter: u8,
    /// roles in the guild
    pub roles: Vec<RoleInfo>,
    /// custom guild emojis
    pub emojis: Vec<EmojiInfo>,
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


    // description: Option<String>,
    // max_members: u32,
    // afk_channels_id: Option<String>,
    // roles: Vec<RoleInfo>,
    // lazy: bool,
    // channels: Vec<ChannelInfo>,
    // TODO: Continue with channels inside guilds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleInfo {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    /// the id of this channel
    pub id: String,
    /// the type of channel
    #[serde(rename="type")]
    pub channel_type: ChannelTypes,
    /// the id of the guild
    pub guild_id: Option<String>,
    /// sorting position of the channel
    pub position: Option<i32>,
    /// explicit permission overwrites for members and roles
    pub permission_overwrites: Vec<PermissionOverwrite>,
    // the name of the channel (2-100 characters)
    pub name: Option<String>,
    /// the channel topic (0-1024 characters)
    pub topic: Option<String>,
    /// whether the channel is nsfw
    pub nsfw: Option<bool>,
    /// the id of the last message sent in this channel (may not point to an existing or valid message)
    pub last_message_id: Option<String>,
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
    pub owner_id: Option<String>,
    /// application id of the group DM creator if it is bot-created
    pub application_id: Option<String>,
    /// id of the parent category for a channel (each parent category can contain up to 50 channels)
    pub parent_id: Option<String>,
    /// when the last pinned message was pinned. This may be null in events such as GUILD_CREATE when a message is not pinned.
    last_pin_timestamp: Option<String>
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EmojiInfo {
    /// emoji id
    pub id: Option<String>,
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

/// https://discord.com/developers/docs/resources/guild#guild-member-object
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMemberInfo {
    /// the user this guild member represents
    pub user: Option<UserInfo>,
    /// this users guild nickname
    pub nick: Option<String>,
    /// array of [role](https://discord.com/developers/docs/topics/permissions#role-object) object ids
    pub roles: Vec<String>,
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
pub struct ActivityInfo {
    /// the activity's name
    pub name: String,
    /// activity type
    #[serde(rename="type")]
    pub activiy_type: ActivityType,
    /// stream url, is validated when type is 1
    
    /// unix timestamp of when the activity was added to the user's session
    
    /// unix timestamps for start and/or end of the game
    
    /// application id for the game
    
    /// what the player is currently doing
    
    /// the user's current party status
    
    /// the emoji used for a custom status
    
    /// information for the current party of the player
    
    /// images for the presence and their hover texts
    
    /// secrets for Rich Presence joining and spectating
    
    /// whether or not the activity is an instanced game session
    
    /// activity flags ORd together, describes what the payload includes

}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwrite {
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

/// https://discord.com/developers/docs/topics/gateway#presence-update
#[derive(Debug, Serialize, Deserialize)]
pub struct PresenceUpdateEventInfo {
    /// the user presence is being updated for
    pub user: UserInfo,
    /// id of the guild
    pub guild_id: String,
    /// either "idle", "dnd", "online", or "offline"
    pub status: String,
    /// user's current activities
    pub activities: Vec<ActivityInfo>,
    /// user's platform-dependent status
    pub client_status: ClientStatusInfo
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

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationLevel {
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

/// https://discord.com/developers/docs/topics/gateway#activity-object-activity-types
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
    Competing = 5
}

bitflags! {
    pub struct PermissionFlags: u64 {
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

bitflags! {
    #[derive(Serialize, Deserialize)]
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

pub enum Result {
    /// everything is good
    Ok = 0,
    /// Discord isn't working
    ServiceUnavailable = 1,
    /// the SDK version may be outdated
    InvalidVersion = 2,
    /// an internal error on transactional operations
    LockFailed = 3,
    /// something on our side went wrong
    InternalError = 4,
    /// the data you sent didn't match what we expect
    InvalidPayload = 5,
    /// that's not a thing you can do
    InvalidCommand = 6,
    /// you aren't authorized to do that
    InvalidPermissions = 7,
    /// couldn't fetch what you wanted
    NotFetched = 8,
    /// what you're looking for doesn't exist
    NotFound = 9,
    /// user already has a network connection open on that channel
    Conflict = 10,
    /// activity secrets must be unique and not match party id
    InvalidSecret = 11,
    /// join request for that user does not exist
    InvalidJoinSecret = 12,
    /// you accidentally set an ApplicationId in your UpdateActivity() payload
    NoEligibleActivity = 13,
    /// your game invite is no longer valid
    InvalidInvite = 14,
    /// the internal auth call failed for the user, and you can't do this
    NotAuthenticated = 15,
    /// the user's bearer token is invalid
    InvalidAccessToken = 16,
    /// access token belongs to another application
    ApplicationMismatch	= 17,
    /// something internally went wrong fetching image data
    InvalidDataUrl = 18,
    /// not valid Base64 data
    InvalidBase64 = 19,
    /// you're trying to access the list before creating a stable list with Filter()
    NotFiltered	= 20,
    /// the lobby is full
    LobbyFull = 21,
    /// the secret you're using to connect is wrong
    InvalidLobbySecret = 22,
    /// file name is too long
    InvalidFilename	= 23,
    /// file is too large
    InvalidFileSize	= 24,
    /// the user does not have the right entitlement for this game
    InvalidEntitlement = 25,
    /// Discord is not installed
    NotInstalled = 26,
    /// Discord is not running
    NotRunning = 27,
    /// insufficient buffer space when trying to write
    InsufficientBuffer = 28,
    /// user cancelled the purchase flow
    PurchaseCancelled = 29,
    /// Discord guild does not exist
    InvalidGuild = 30,
    /// the event you're trying to subscribe to does not exist
    InvalidEvent = 31,
    /// Discord channel does not exist
    InvalidChannel = 32,
    /// the origin header on the socket does not match what you've registered (you should not see this)
    InvalidOrigin = 33,
    /// you are calling that method too quickly
    RateLimited	= 34,
    /// the OAuth2 process failed at some point
    OAuth2Error	= 35,
    /// the user took too long selecting a channel for an invite
    SelectChannelTimeout = 36, 
    /// took too long trying to fetch the guild
    GetGuildTimeout	= 37,
    /// push to talk is required for this channel
    SelectVoiceForceRequired = 38,
    /// that push to talk shortcut is already registered
    CaptureShortcutAlreadyListening = 39,
    /// your application cannot update this achievement
    UnauthorizedForAchievement = 40;
    /// the gift code is not valid
    InvalidGiftCode	= 41,
    /// something went wrong during the purchase flow
    PurchaseError = 42,
    /// purchase flow aborted because the SDK is being torn down
    TransactionAborted = 43,
}
