use axum::body::Bytes;
use axum::extract::Path;
use axum::http::header::HeaderMap;
use axum::response::IntoResponse;
use ed25519_compact::{PublicKey, Signature};
use http::StatusCode;

use std::env;
pub use translation::*;

use model::{Interaction, InteractionResponse, InteractionResponseType, InteractionType, Message};

use handler_error::HandleError;
use handler_error::HandleError::AuthenticationForbiden;
use handler_error::HandleError::Internal;

mod handler_error;
mod model;
mod translation;

pub async fn handle_interaction(
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let body2 = body.clone();
    if let Err(e) = validate(body, &headers).await {
        match e {
            HandleError::AuthenticationForbiden => {
                return Err((StatusCode::UNAUTHORIZED, "unauthorized".to_string()))
            }
            _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
    response_interaction(body2)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

pub async fn validate(body: Bytes, headers: &HeaderMap) -> Result<(), HandleError> {
    let sig = headers
        .get("X-Signature-Ed25519")
        .ok_or_else(|| Internal("cannot get X-Signature-Ed25519".to_string()))
        .and_then(|sig| hex::decode(sig).map_err(|e| Internal(e.to_string())))
        .and_then(|mm| Signature::from_slice(&mm).map_err(|e| Internal(e.to_string())))?;

    let timestamp = headers
        .get("X-Signature-Timestamp")
        .ok_or_else(|| Internal("cannot get X-Signature-Timestamp".to_string()))?;
    let content = vec![timestamp.as_bytes(), &body].concat();

    let discord_pub_key = env::var("DISCORD_PUBLIC_KEY").map_err(|e| Internal(e.to_string()))?;
    let discord_pub_key =
        hex::decode(discord_pub_key).map_err(|_e| Internal("cannot decode token".to_string()))?;

    let pk = PublicKey::from_slice(&discord_pub_key)
        .map_err(|_e| Internal("cannot get pub key".to_string()))?;

    pk.verify(content, &sig)
        .map_err(|_e| AuthenticationForbiden)?;

    Ok(())
}

async fn response_interaction(body: Bytes) -> Result<impl IntoResponse, HandleError> {
    let i = bind_interaction(body)?;
    match i.interaction_type {
        InteractionType::Ping => InteractionResponse {
            interaction_response_type: InteractionResponseType::Pong,
            data: None,
        }
        .into_response()
        .map_err(HandleError::ParseResponse),
        InteractionType::ApplicationCommand => {
            println!("{:?}", i);
            let messages = i.data.unwrap().resolved.unwrap().messages.unwrap();
            let msgs: Vec<&Message> = messages.iter().map(|(_key, msg)| msg).collect();
            let msg = msgs[0];

            let deepl_api_key = env::var("DEEPL_API_KEY")
                .map_err(|e| HandleError::NotFoundSecret(e.to_string()))?;

            let translator = translation::Translator::new(&deepl_api_key);
            let text = translator
                .translate(&msg.content)
                .await
                .map_err(HandleError::FailedTranslation)?;

            InteractionResponse::reply(format!("`{}` ->\n{}", &msg.content, text))
                .into_response()
                .map_err(HandleError::ParseResponse)
        }
    }
}

fn bind_interaction(body: Bytes) -> Result<Interaction, HandleError> {
    serde_json::from_slice::<Interaction>(&body).map_err(|_e| HandleError::Parse)
}

pub async fn translate_to_japanese(
    Path(text): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let deepl_api_key =
        env::var("DEEPL_API_KEY").map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let translator = translation::Translator::new(&deepl_api_key);
    let translated = translator
        .translate(&text)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    InteractionResponse::reply(format!("`{}` ->\n{}", text, translated))
        .into_response()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot reply response error:{}", e),
            )
        })
}
