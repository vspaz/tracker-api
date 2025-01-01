use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseOk {
    pub status: String,
    pub message: String,
}

pub async fn track(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}

pub async fn page(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}

pub async fn identify(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}

pub async fn alias(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}

pub async fn screen(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}

pub async fn batch(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}
