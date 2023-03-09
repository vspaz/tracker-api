use std::collections::HashMap;
use actix_web::{HttpRequest, HttpResponse, Responder, web};

#[warn(unused_variables)]
pub async fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(HashMap::from([("ping", "pong")]))
}
