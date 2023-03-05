use actix_web::{HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub async fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
