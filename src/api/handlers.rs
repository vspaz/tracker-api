use crate::api::health;
use crate::api::segment;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::{get, post};
use actix_web::{App, Error};

fn with_api_prefix(endpoint: &str) -> String {
    "/api/v1/".to_owned() + endpoint
}

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
        .route(&*with_api_prefix("track"), post().to(segment::track))
        .route(&*with_api_prefix("t"), post().to(segment::track))
        .route(&*with_api_prefix("page"), post().to(segment::page))
        .route(&*with_api_prefix("p"), post().to(segment::page))
        .route(&*with_api_prefix("identify"), post().to(segment::identify))
        .route(&*with_api_prefix("i"), post().to(segment::identify))
        .route(&*with_api_prefix("alias"), post().to(segment::alias))
        .route(&*with_api_prefix("a"), post().to(segment::alias))
        .route(&*with_api_prefix("screen"), post().to(segment::screen))
        .route(&*with_api_prefix("s"), post().to(segment::screen))
        .route(&*with_api_prefix("b"), post().to(segment::batch))
        // service endpoint
        .route("/ping/", get().to(health::ping))
        .route("/", get().to(health::ping))
}
