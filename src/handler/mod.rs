use ed25519_compact::{PublicKey, Signature};

use model::{Interaction, InteractionResponse, InteractionResponseType, InteractionType, Message};
use worker::*;
mod model;

pub async fn handle_su_shan_shan(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Err(_e) = validate(req.clone()?, ctx).await {
        return Response::error("unauthorized", 401);
    }
    response_interaction(req).await
}

async fn validate(mut req: Request, ctx: RouteContext<()>) -> Result<bool> {
    let body: Vec<u8> = req.bytes().await?;
    let hd = req.headers();

    let sig = hd
        .get("X-Signature-Ed25519")?
        .ok_or_else(|| "cannot get X-Signature-Ed25519".to_string())
        .and_then(|sig| hex::decode(sig).map_err(|e| e.to_string()))
        .and_then(|mm| Signature::from_slice(&mm).map_err(|e| e.to_string()))?;

    let timestamp = hd
        .get("X-Signature-Timestamp")?
        .ok_or("cannot get X-Signature-Timestamp")?;
    let content = vec![timestamp.as_bytes(), &body].concat();

    let discord_pub_key = ctx.secret("DISCORD_PUBLIC_KEY")?.to_string();
    let discord_pub_key = hex::decode(discord_pub_key).map_err(|_e| "cannot decode token")?;

    let pk = PublicKey::from_slice(&discord_pub_key).map_err(|_e| "cannot get pub key")?;

    pk.verify(content, &sig)
        .map_err(|e| Error::RustError(e.to_string()))?;

    Ok(true)
}

async fn response_interaction(mut req: Request) -> Result<Response> {
    let i = bind_interaction(&mut req).await?;
    match i.interaction_type {
        InteractionType::Ping => InteractionResponse {
            interaction_response_type: InteractionResponseType::Pong,
            data: None,
        }
        .into_response(),
        InteractionType::ApplicationCommand => {
            // console_log!("{:?}", i);
            let messages = i.data.unwrap().resolved.unwrap().messages.unwrap();
            let msgs: Vec<&Message> = messages.iter().map(|(_key, msg)| msg).collect();
            let msg = msgs[0];
            InteractionResponse::reply(msg.content.clone()).into_response()
        }
    }
}

async fn bind_interaction(req: &mut Request) -> Result<Interaction> {
    let byte = req.bytes().await.map_err(|e| e.to_string())?;
    let i = serde_json::from_slice::<Interaction>(byte.as_ref()).map_err(|e| e.to_string())?;
    Ok(i)
}
