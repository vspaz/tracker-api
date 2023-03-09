use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::api::handlers::ResponseOk;

#[warn(unused_variables)]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    });
}
