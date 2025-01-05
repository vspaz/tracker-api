use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseOk {
    pub status: String,
    pub message: String,
}

pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}
