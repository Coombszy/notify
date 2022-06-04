use crate::libs::structs::{State, WebHealth, Notification};

use futures_util::StreamExt as _;
use actix_web::{get, post, web, HttpResponse, Error, error};
use log::debug;

const MAX_PAYLOAD_SIZE: usize = 262_144; // Max size of 256k

#[get("/health")]
async fn health(data: web::Data<State>) -> HttpResponse {
    debug!("Health request received");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}

#[post("/notification")]
async fn notification(data: web::Data<State>, mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // Convert payload stream into useful object
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("payload overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<Notification>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}