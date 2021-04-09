use serde::{Deserialize, Serialize};

/// https://discord.com/developers/docs/resources/channel#embed-object
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedInfo {
    /// title of embed
    pub title: Option<String>,
    /// [type of embed](https://discord.com/developers/docs/resources/channel#embed-object-embed-types) (always "rich" for webhook embeds)
    #[serde(rename="type")]
    pub embed_type: Option<String>,
    /// description of embed
    pub description: Option<String>,
    /// url of embed
    pub url: Option<String>,
    /// timestamp of embed content
    pub timestamp: Option<String>,
    /// color code of the embed
    pub color: Option<i32>,
    /// footer information
    pub footer: Option<EmbedFooterInfo>,
    /// image information
    pub image: Option<EmbedMediaInfo>,
    /// thumbnail information
    pub thumbnail: Option<EmbedMediaInfo>,
    /// video information
    pub video: Option<EmbedMediaInfo>,
    /// provider information
    pub provider: Option<EmbedProviderInfo>,
    /// author information
    pub author: Option<EmbedMediaInfo>,
    /// fields information
    pub fields: Option<Vec<EmbedFieldInfo>>
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooterInfo {
    /// footer text
    pub text: String,
    /// url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// a proxied url of footer icon
    pub proxy_icon_url: Option<String>
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedMediaInfo {
    /// source url of image (only supports http(s) and attachments)
    #[serde(alias="icon_url")]
    pub url: Option<String>,
    /// a proxied url of the image
    #[serde(alias="proxy_icon_url")]
    pub proxy_url: Option<String>,
    /// height of image
    pub height: Option<i32>,
    /// width of image
    pub width: Option<i32>,
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProviderInfo {
    /// name of provider
    pub name: Option<String>,
    /// url of provider
    pub url: Option<String>
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFieldInfo {
    /// name of the field
    pub name: String,
    /// value of the field
    pub value: String,
    /// whether or not this field should display inline
    pub inline: Option<bool>
}