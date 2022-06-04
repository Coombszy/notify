use crate::libs::structs::{State, WebHealth};
use actix_web::{get, web, HttpResponse};
use log::debug;

#[get("/health")]
async fn health(data: web::Data<State>) -> HttpResponse {
    debug!("Health request received");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}
