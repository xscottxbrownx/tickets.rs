use serde::{Serialize, Deserialize, Deserializer};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::interaction::InteractionApplicationCommandCallbackData;
use std::convert::TryFrom;
use serde_json::Value;
use serde::de::Error;

// TODO: Reduce redundant code

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InteractionResponse {
    PongResponse(SimpleInteractionResponse),
    ChannelMessageWithSource(ApplicationCommandResponse),
    DeferredChannelMessageWithSource(DeferredApplicationCommandResponse),
    DeferredMessageUpdate(SimpleInteractionResponse),
    // UpdateMessage is not yet supported
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleInteractionResponse {
    r#type: InteractionResponseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandResponse {
    r#type: InteractionResponseType,
    data: InteractionApplicationCommandCallbackData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeferredApplicationCommandResponse {
    r#type: InteractionResponseType,
    data: DeferredApplicationCommandResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeferredApplicationCommandResponseData {
    pub flags: usize,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredMessageUpdate = 6,
    UpdateMessage = 7,
}

impl TryFrom<u64> for InteractionResponseType {
    type Error = Box<str>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Pong,
            4 => Self::ChannelMessageWithSource,
            5 => Self::DeferredChannelMessageWithSource,
            6 => Self::DeferredMessageUpdate,
            7 => Self::UpdateMessage,
            _ => Err(format!("invalid interaction response type \"{}\"", value).into_boxed_str())?
        })
    }
}

impl InteractionResponse {
    pub fn new_pong() -> InteractionResponse {
        InteractionResponse::PongResponse(SimpleInteractionResponse {
            r#type: InteractionResponseType::Pong,
        })
    }

    pub fn new_channel_message_with_source(data: InteractionApplicationCommandCallbackData) -> InteractionResponse {
        InteractionResponse::ChannelMessageWithSource(ApplicationCommandResponse {
            r#type: InteractionResponseType::ChannelMessageWithSource,
            data,
        })
    }

    pub fn new_deferred_message_with_source() -> InteractionResponse {
        InteractionResponse::DeferredChannelMessageWithSource(DeferredApplicationCommandResponse {
            r#type: InteractionResponseType::DeferredChannelMessageWithSource,
            data: DeferredApplicationCommandResponseData {
                flags: 64,
            }
        })
    }

    pub fn new_deferred_message_update() -> InteractionResponse {
        InteractionResponse::DeferredMessageUpdate(SimpleInteractionResponse {
            r#type: InteractionResponseType::DeferredMessageUpdate,
        })
    }
}

impl<'de> Deserialize<'de> for InteractionResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;

        let response_type = value.get("type")
            .and_then(Value::as_u64)
            .ok_or_else(|| Box::from("interaction response type was not an integer"))
            .and_then(InteractionResponseType::try_from)
            .map_err(D::Error::custom)?;

        let response = match response_type {
            InteractionResponseType::Pong => serde_json::from_value(value).map(InteractionResponse::PongResponse),
            InteractionResponseType::ChannelMessageWithSource => serde_json::from_value(value).map(InteractionResponse::ChannelMessageWithSource),
            InteractionResponseType::DeferredChannelMessageWithSource => serde_json::from_value(value).map(InteractionResponse::DeferredChannelMessageWithSource),
            InteractionResponseType::DeferredMessageUpdate => serde_json::from_value(value).map(InteractionResponse::DeferredMessageUpdate),
            InteractionResponseType::UpdateMessage => Err(Error::custom("UpdateMessage is not yet supported")),
        }.map_err(D::Error::custom)?;

        Ok(response)
    }
}
