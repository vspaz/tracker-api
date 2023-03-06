use actix_web::{HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub async fn track(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn t(_req: HttpRequest) -> impl Responder {
    track(_req)
}

#[warn(unused_variables)]
pub async fn page(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn p(_req: HttpRequest) -> impl Responder {
    track(_req)
}