use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseOk {
    pub status: String,
    pub message: String,
}

#[warn(unused_variables)]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}
