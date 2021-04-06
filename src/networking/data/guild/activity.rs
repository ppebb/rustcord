use serde::{Deserialize, Serialize};
use serde_repr::*;

use super::super::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityInfo {
    /// the activity's name
    pub name: String,
    /// activity type
    #[serde(rename="type")]
    pub activity_type: ActivityType,
    /// stream url, is validated when type is 1
    pub url: Option<String>,
    /// unix timestamp of when the activity was added to the user's session
    pub created_at: u64,
    /// unix timestamps for start and/or end of the game
    pub timestamps: ActivityTimestamps,
    /// application id for the game
    pub application_id: Option<Snowflake>,
    /// what the player is currently doing
    pub details: Option<String>,
    /// the user's current party status
    pub state: Option<String>,
    /// the emoji used for a custom status
    pub emoji: Option<ActivityEmoji>,
    /// information for the current party of the player
    pub party: Option<ActivityParty>,
    /// images for the presence and their hover texts
    pub assets: Option<ActivityAssets>,
    /// secrets for Rich Presence joining and spectating
    pub secrets: Option<ActivitySecrets>,
    /// whether or not the activity is an instanced game session
    pub instance: Option<bool>,
    /// activity flags ORd together, describes what the payload includes
    pub flags: Option<ActivityFlags>,
    pub id: Option<String>,
}

/// https://discord.com/developers/docs/topics/gateway#activity-object-activity-timestamps
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    /// unix time (in milliseconds) of when the activity started
    pub start: Option<u64>,
    /// unix time (in milliseconds) of when the activity ends
    pub end: Option<u64>
}

/// https://discord.com/developers/docs/topics/gateway#activity-object-activity-emoji
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityEmoji {
    /// the name of the emoji
    pub name: String,
    /// the id of the emoji
    pub id: Option<Snowflake>,
    /// whether this emoji is animated
    pub animated: Option<bool>
}

/// https://discord.com/developers/docs/topics/gateway#activity-object-activity-party
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityParty {
    /// the id of the party
    pub id: Option<String>,
    /// used to show the party's current and maximum size
    pub size: Option<Vec<i32>>
}

/// https://discord.com/developers/docs/topics/gateway#activity-object-activity-assets
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityAssets {
    /// the id for a large asset of the activity, usually a snowflake
    pub large_image: Option<String>,
    /// text displayed when hovering over the large image of the activity
    pub large_text: Option<String>,
    /// the id for a small asset of the activity, usually a snowflake
    pub small_image: Option<String>,
    /// text displayed when hovering over the small image of the activity
    pub small_text: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySecrets {
    /// the secret for joining a party
    pub join: Option<String>,
    /// the secret for spectating a game
    pub spectate: Option<String>,
    /// the secret for a specific instanced match
    #[serde(rename="match")]
    pub secret_match: Option<String> 
}

/// https://discord.com/developers/docs/topics/gateway#activity-object-activity-types
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
    Competing = 5
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ActivityFlags: u8 {
        const INTANCE = 1 << 0;
        const JOIN = 1 << 1;
        const SPECTATE = 1 << 2;
        const JOIN_REQUEST = 1 << 3;
        const SYNC = 1 << 4;
        const PLAY = 1 << 5;
    }
}