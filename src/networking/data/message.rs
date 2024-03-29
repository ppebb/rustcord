use serde_json::Value;
use serde_repr::*;
use serde::{Deserialize, Serialize};
use self::embed::EmbedInfo;

use super::{Snowflake, channel::ChannelTypes, guild::{GuildEmojiInfo, GuildMemberInfo}, user::UserInfo};

pub mod embed;

/// https://discord.com/developers/docs/resources/channel#message-object
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInfo {
    /// id of the message
    pub id: Snowflake,
    /// id of the channel the message was sent in
    pub channel_id: Snowflake,
    /// id of the guild the message was sent in
    pub guild_id: Option<Snowflake>,
    /// the author of this message (not guaranteed to be a valid user, see below)
    pub author: UserInfo,
    /// member properties for this message's author
    pub member: Option<GuildMemberInfo>,
    /// contents of the message
    pub content: String,
    /// when this message was sent
    pub timestamp: String,
    /// when this message was edited (or null if never)
    pub edited_timestamp: Option<String>,
    /// whether this was a TTS message
    pub tts: bool,
    /// whether this message mentions everyone
    pub mention_everyone: bool,
    /// users specifically mentioned in the message
    pub mentions: Vec<Value>, // TODO
    /// roles specifically mentioned in this message
    pub mention_roles: Vec<Snowflake>,
    /// channels specifically mentioned in this message
    pub mention_channels: Option<Vec<ChannelMentionInfo>>,
    /// any attached files
    pub attachments: Vec<MessageAttachmentInfo>,
    /// any embedded content
    pub embeds: Vec<EmbedInfo>,
    /// reactions to the message
    pub reactions: Option<Vec<ReactionInfo>>,
    /// used for validating a message was sent.
    /// integer or string
    pub nonce: Option<Value>,
    /// whether this message is pinned
    pub pinned: bool,
    /// if the message is generated by a webhook, this is the webhook's id
    pub webhook_id: Option<Snowflake>,
    /// [type of message](https://discord.com/developers/docs/resources/channel#message-object-message-types)
    #[serde(rename="type")]
    pub message_type: MessageType,
    /// sent with Rich Presence-related chat embeds
    pub activity: Option<MessageActivity>,
    /// sent with Rich Presence-related chat embeds
    pub application: Option<MessageApplication>,
    /// reference data sent with crossposted messages and replies
    pub message_reference: Option<MessageReference>,
    /// message flags combined as a bitfield
    pub flags: Option<MessageFlags>,
    /// the stickers sent with the message (bots currently can only receive messages with stickers, not send)
    pub stickers: Option<Vec<MessageSticker>>,
    /// the message associated with the message_reference
    pub referenced_message: Option<Box<MessageInfo>>,
    /// sent if the message is a response to an Interaction
    pub interaction: Option<MessageInteraction>
}

/// https://discord.com/developers/docs/resources/channel#channel-mention-object
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelMentionInfo {
    /// id of the channel
    pub id: Snowflake,
    /// id of the guild containing the channel
    pub guild_id: Snowflake,
    /// the [type of channel](https://discord.com/developers/docs/resources/channel#channel-object-channel-types)
    #[serde(rename="type")]
    pub channel_type: ChannelTypes,
    /// the name of the channel
    pub name: String 
}

/// https://discord.com/developers/docs/resources/channel#attachment-object
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageAttachmentInfo {
    /// attachment id
    pub id: Snowflake,
    /// name of file attached
    pub filename: String,
    /// size of file in bytes
    pub size: i32,
    /// source url of file
    pub url: String,
    /// a proxied url of file
    pub proxy_url: String,
    /// height of file (if image)
    pub height: Option<i32>,
    /// width of file (if image)
    pub width: Option<i32>
}

/// https://discord.com/developers/docs/resources/channel#reaction-object
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionInfo {
    /// times this emoji has been used to react
    pub count: i32,
    /// whether the current user reacted using this emoji
    pub me: bool,
    /// emoji information
    pub emoji: GuildEmojiInfo
}

/// https://discord.com/developers/docs/resources/channel#message-object-message-types
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconChange,
    ChannelPinnedMessage,
    GuildMemberJoin,
    UserPremiumGuildSubscription,
    UserPremiumGuildSubscriptionTier1,
    UserPremiumGuildSubscriptionTier2,
    UserPremiumGuildSubscriptionTier3,
    ChannelFollowAdd,
    GuildDiscoveryDisqualified,
    GuildDiscoveryRequalified,
    Reply,
    ApplicationCommand
}

/// https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageActivity {
    /// type of message activity
    #[serde(rename="type")]
    pub activity_type: MessageActivityType,
    /// party_id from a [Rich Presence event](https://discord.com/developers/docs/rich-presence/how-to#updating-presence-update-presence-payload-fields)
    pub party_id: Option<String>
}

/// https://discord.com/developers/docs/resources/channel#message-object-message-activity-types
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5
}

/// https://discord.com/developers/docs/resources/channel#message-object-message-application-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageApplication {
    /// id of the application
    pub id: Snowflake,
    /// id of the embed's image asset
    pub cover_image: Option<String>,
    /// application's description
    pub description: String,
    /// id of the application's icon
    pub icon: Option<String>,
    /// name of the application
    pub name: String
}

/// https://discord.com/developers/docs/resources/channel#message-object-message-reference-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageReference {
    /// id of the originating message
    pub message_id: Option<Snowflake>,
    /// id of the originating message's channel
    pub channel_id: Option<Snowflake>,
    /// id of the originating message's guild
    pub guild_id: Option<Snowflake>,
    /// when sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
    pub fail_if_not_exists: Option<bool>
}

bitflags! {
    /// https://discord.com/developers/docs/resources/channel#message-object-message-flags
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct MessageFlags: u8 {
        /// this message has been published to subscribed channels (via Channel Following)
        const CROSSPOSTED = 1 << 0;
        /// this message originated from a message in another channel (via Channel Following)
        const IS_CROSSPOST = 1 << 1;
        /// do not include any embeds when serializing this message
        const SUPRESS_EMBEDS = 1 << 2;
        /// the source message for this crosspost has been deleted (via Channel Following)
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// this message came from the urgent message system
        const URGENT = 1 << 4;
    }
}

/// https://discord.com/developers/docs/resources/channel#message-object-message-sticker-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSticker {
    /// id of the sticker
    pub id: Snowflake,
    /// id of the pack the sticker is from
    pub pack_id: Snowflake,
    /// name of the sticker
    pub name: String,
    /// description of the sticker
    pub description: String,
    /// a comma-separated list of tags for the sticker
    pub tags: Option<String>,
    /// sticker asset hash
    pub asset: String,
    /// sticker preview asset hash
    pub preview_asset: Option<String>,
    /// [type of sticker format](https://discord.com/developers/docs/resources/channel#message-object-message-sticker-format-types)
    pub format_type: u8
}

/// https://discord.com/developers/docs/interactions/slash-commands#messageinteraction
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInteraction {
    /// id of the interaction
    pub id: Snowflake,
    /// the type of interaction
    #[serde(rename="type")]
    pub interaction_type: i32,
    /// the name of the ApplicationCommand
    pub name: String,
    /// the user who invoked the interaction
    pub user: UserInfo
}