use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;

pub async fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(HashMap::from([("ping", "pong")]))
}
