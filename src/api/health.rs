use actix_web::{HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub(crate) async fn ping(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}