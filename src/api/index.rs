use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseOk {
    status: String,
    message: String,
}

#[warn(unused_variables)]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk{ status: "200 OK".to_string(), message: "OK".to_string() })
}
