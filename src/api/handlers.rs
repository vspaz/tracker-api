use crate::api::health::ping;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::{get, post};
use actix_web::{App, Error};
use crate::api::segment::track;

pub fn register_handlers() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .route("/ping/", get().to(ping))
        .route("/", get().to(ping))
        .route("/track", post().to(track))
}
