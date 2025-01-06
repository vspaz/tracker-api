use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseOk {
    pub status: String,
    pub message: String,
}

pub async fn index(_req: HttpRequest) -> impl Responder {
    let response = ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    };
    HttpResponse::Ok().json(response)
}
