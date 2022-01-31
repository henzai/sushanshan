use std::collections::HashMap;

use http::Response;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use serde_json::Value;

type Snowflake = String;

#[derive(Clone, Debug, Deserialize)]
pub struct Interaction {
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<ApplicationCommandInteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub token: String,
    pub version: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMember {
    pub deaf: bool,
    pub nick: Option<String>,
    pub roles: Vec<String>,
    /// Attached User struct.
    pub user: User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub discriminator: String,
    pub username: String,
}
#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationCommandInteractionData {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub application_command_type: ApplicationCommandType,
    pub resolved: Option<ApplicationCommandInteractionDataResolved>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub custom_id: Option<String>,
    pub component_type: Option<ComponentType>,
    pub target_id: Snowflake,
}

#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
}

#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    #[serde(rename = "type")]
    pub application_command_option_type: ApplicationCommandOptionType,
    pub value: Option<Value>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub focused: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationCommandInteractionDataResolved {
    pub messages: Option<HashMap<String, Message>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    Subcommand = 1,
    Subcommandgroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
}

#[derive(Clone, Debug, Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub interaction_response_type: InteractionResponseType,
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

impl InteractionResponse {
    pub fn reply(content: String) -> InteractionResponse {
        InteractionResponse {
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionApplicationCommandCallbackData {
                tts: None,
                content: Some(content),
                flags: None,
            }),
        }
    }
    pub fn into_response(self) -> Result<Response<Body>, String> {
        let sss = serde_json::to_string(&self).map_err(|e| e.to_string())?;
        Response::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(sss.into())
            .map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize_repr)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    // Acknowledge = 2,
    // ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}

#[derive(Clone, Debug, Serialize)]
pub struct InteractionApplicationCommandCallbackData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    // embeds
    // allowed_mentions
    pub flags: Option<usize>,
}
