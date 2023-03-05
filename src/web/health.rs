use actix_web::{HttpRequest, HttpResponse, Responder};

async fn ping(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}