use actix_web::{HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;

pub async fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(HashMap::from([("ping", "pong")]))
}
