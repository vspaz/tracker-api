use crate::api::health::ping;
use crate::api::segment::{alias, identify, page, track};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::{get, post};
use actix_web::{App, Error};

fn add_api_prefix_endpoint(endpoint: &str) -> String {
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
        .route(&*add_api_prefix_endpoint("track"), post().to(track))
        .route(&*add_api_prefix_endpoint("t"), post().to(track))
        .route(&*add_api_prefix_endpoint("page"), post().to(page))
        .route(&*add_api_prefix_endpoint("p"), post().to(page))
        .route(&*add_api_prefix_endpoint("identify"), post().to(identify))
        .route(&*add_api_prefix_endpoint("i"), post().to(identify))
        .route(&*add_api_prefix_endpoint("alias"), post().to(alias))
        .route(&*add_api_prefix_endpoint("a"), post().to(alias))
        // service endpoint
        .route("/ping/", get().to(ping))
        .route("/", get().to(ping))
}
