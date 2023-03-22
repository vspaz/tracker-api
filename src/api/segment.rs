use crate::api::handlers::ResponseOk;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[warn(unused_variables)]
pub async fn track(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[warn(unused_variables)]
pub async fn page(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    });
}

#[warn(unused_variables)]
pub async fn identify(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    });
}

#[warn(unused_variables)]
pub async fn alias(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    });
}

#[warn(unused_variables)]
pub async fn screen(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    });
}

#[warn(unused_variables)]
pub async fn batch(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish();
    return web::Json(ResponseOk {
        status: "200 OK".to_string(),
        message: "OK".to_string(),
    });
}

