use crate::libs::structs::{Notification, State, WebError, WebHealth};

use actix_web::{error, get, post, web, Error, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt as _;
use log::debug;

// Compile using mocked IFTTT Sending
#[cfg(not(debug_assertions))]
use crate::libs::utils::send_notification;
#[cfg(debug_assertions)]
use crate::mocks::utils::send_notification;

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
async fn notification(
    data: web::Data<State>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // Convert payload stream into useful object
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("payload overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let mut notification = match serde_json::from_slice::<Notification>(&body) {
        Ok(n) => n,
        Err(e) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .json(WebError {
                    timestamp: Utc::now().to_rfc3339(),
                    error: format!("failed to parse json. {}", e),
                }));
        }
    };

    if notification.title.is_empty() || notification.content.is_empty() {
        return Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .json(WebError {
                timestamp: Utc::now().to_rfc3339(),
                error: "notification json must contain `title` and `content`. `image` is optional"
                    .to_string(),
            }));
    }
    debug!(
        "Web notification received: {}",
        serde_json::to_string(&notification).unwrap()
    );

    notification.key = Some(data.key.clone());
    notification.event = Some(data.event.clone());

    send_notification(notification.clone()).await;
    Ok(HttpResponse::NoContent().finish())
}

/*
########################################################################################################
#   Copyright (C) 2022 Coombszy
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
