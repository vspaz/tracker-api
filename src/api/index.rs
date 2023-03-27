use crate::api::handlers::ResponseOk;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    })
}
