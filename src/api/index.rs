use actix_web::{HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
