use actix_web::{HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub async fn track(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn page(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn identify(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn alias(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn screen(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
